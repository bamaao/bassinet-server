use std::{env, sync::Arc};
use sea_orm::{ActiveValue::Set, ModelTrait};
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

use crate::{application::command_service::sui_application_service, domain::{model::entity::bassinet_coin, repository::{account_repository, bassinet_coin_repository}}};

use super::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinPublishedMessage {
    pub package_id: String,
    pub treasury_lock_id: String,
    pub admin_cap_id: String,
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub icon_url: String,
    pub account: String,
    pub wallet_address: String,
}

/// This function is the long-running RabbitMQ task.
/// It starts the RabbitMQ client, and if it fails, it will attempt to reconnect.
pub async fn coin_published_consumer(cfg: Arc<Config>) -> anyhow::Result<()> {
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
        .queue_declare(QueueDeclareArguments::durable_client_named("sui_coin_published_event").durable(true).exclusive(true).auto_delete(false).finish())
        .await
        .context("failed to declare queue")?
        .expect("when no_wait is false (default) then we should have a value");
    debug!("declared queue '{queue_name}'");

    let exchange_name = "bassinet.topic";
    debug!("binding exchange {exchange_name} -> queue {queue_name}");
    channel
        .queue_bind(QueueBindArguments::new(&queue_name, exchange_name, "bassinet.CoinPublished"))
        .await
        .context("queue binding failed")?;

    let consume_args = BasicConsumeArguments::new(&queue_name, "CoinPublished").auto_ack(false).finish();
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
            let coin_info: CoinPublishedMessage = serde_json::from_str(json).unwrap();

            let result = sui_application_service::add_bassinet_coin(&coin_info).await;
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