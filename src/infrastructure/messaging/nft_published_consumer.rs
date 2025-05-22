use std::{env, sync::Arc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use amqprs::{
    callbacks::{DefaultChannelCallback, DefaultConnectionCallback},
    channel::{
        BasicAckArguments, BasicCancelArguments, BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments
    },
    connection::{Connection, OpenConnectionArguments},
};
use anyhow::Context;
use anyhow::anyhow;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, trace, warn};

use crate::{application::command_service::sui_application_service, domain::repository::account_repository};

use super::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct NftPublishedMessage {
    pub collection_id: String,
    pub package_id: String,
    pub mint_id: String,
    pub policy_id: String,
    pub policy_cap_id: String,
    pub coin_package_id: String,
    pub treasury_lock_id: String,
    pub admin_cap_id: String,
    pub description: String,
    pub collection_url: String,
    pub limit: u64,
    pub rewards_quantity: u64,
    pub minting_price: u64,
}

/// This function is the long-running RabbitMQ task.
/// It starts the RabbitMQ client, and if it fails, it will attempt to reconnect.
pub async fn nft_published_consumer(cfg: Arc<Config>) -> anyhow::Result<()> {
    loop {
        let result = process(cfg.clone()).await;
        match result {
            Ok(value) => {
                // Not actually implemented right now.
                warn!("exiting in response to a shutdown command");
                return Ok(value);
            }
            Err(err) => {
                error!("RabbitMQ connection returned error: {err:?}");
                sleep(Duration::from_millis(1000)).await;
                info!("ready to restart consumer task");
            }
        }
    }
}

/// RabbitMQ client task. Returns an error result if the connection is lost.
async fn process(cfg: Arc<Config>) -> anyhow::Result<()> {
    // debug!("starting RabbitMQ task");

    let connection = Connection::open(
        &OpenConnectionArguments::new(&cfg.host, cfg.port, &cfg.username, &cfg.password)
            .virtual_host(&cfg.virtual_host),
    )
    .await
    .with_context(|| {
        format!(
            "can't connect to RabbitMQ server at {}:{}",
            cfg.host, cfg.port
        )
    })?;

    // Add simple connection callback, it just logs diagnostics.
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .context("registering connection callback failed")?;

    let channel = connection
        .open_channel(None)
        .await
        .context("opening channel failed")?;
    channel
        .register_callback(DefaultChannelCallback)
        .await
        .context("registering channel callback failed")?;

    // Declare our receive queue.
    let (queue_name, _, _) = channel
        .queue_declare(QueueDeclareArguments::durable_client_named("sui_nft_published_event").durable(true).exclusive(true).auto_delete(false).finish())
        .await
        .context("failed to declare queue")?
        .expect("when no_wait is false (default) then we should have a value");
    debug!("declared queue '{queue_name}'");

    let exchange_name = "bassinet.topic";
    debug!("binding exchange {exchange_name} -> queue {queue_name}");
    channel
        .queue_bind(QueueBindArguments::new(&queue_name, exchange_name, "bassinet.NftPublished"))
        .await
        .context("queue binding failed")?;

    let consume_args = BasicConsumeArguments::new(&queue_name, "NftPublished").auto_ack(false).finish();
    // let consumer = MyConsumer::new(consume_args.no_ack);
    // let consumer_tag = channel
    //     .basic_consume(consumer, consume_args)
    //     .await
    //     .context("failed basic_consume")?;
    // trace!("consumer tag: {consumer_tag}");

    // if connection.listen_network_io_failure().await {
    //     Err(RabbitError::ConnectionLost("connection failure".to_owned()).into())
    // } else {
    //     Err(RabbitError::ConnectionLost("connection shut down normally. Since we don't close it ourselves, this shouldn't happen in this program".to_owned()).into())
    // }
    let (ctag, mut rx) = channel.basic_consume_rx(consume_args).await.unwrap();
    let new_channel = channel.clone();
    let jh = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let content = msg.content.unwrap();
            let deliver = msg.deliver.unwrap();
            let json = std::str::from_utf8(&content).unwrap();
            info!(
                "consume delivery {}, content: {}",
                deliver,
                json
            );

            let nft_info: NftPublishedMessage = serde_json::from_str(json).unwrap();

            let result = sui_application_service::add_bassinet_nft(&nft_info).await;
            if result.is_err() {
                tracing::error!("message:{}, error:{}", json, result.err().unwrap());
                // TODO 
            }
            // Ack explicitly
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            new_channel.basic_ack(args).await.unwrap();
        }
    });
    assert!(jh.await.is_err());
    channel.basic_cancel(BasicCancelArguments::new(&ctag)).await.unwrap();
    Err(anyhow!("binding account consumer panic"))
}