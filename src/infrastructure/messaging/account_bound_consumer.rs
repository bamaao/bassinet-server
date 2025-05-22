use std::sync::Arc;
use serde_json::Value;

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

use crate::application::command_service::sui_application_service;

use super::Config;

/// This function is the long-running RabbitMQ task.
/// It starts the RabbitMQ client, and if it fails, it will attempt to reconnect.
pub async fn account_bound_consumer(cfg: Arc<Config>) -> anyhow::Result<()> {
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
        .queue_declare(QueueDeclareArguments::durable_client_named("sui_account_bound_event").durable(true).exclusive(true).auto_delete(false).finish())
        .await
        .context("failed to declare queue")?
        .expect("when no_wait is false (default) then we should have a value");
    debug!("declared queue '{queue_name}'");

    let exchange_name = "bassinet.topic";
    debug!("binding exchange {exchange_name} -> queue {queue_name}");
    channel
        .queue_bind(QueueBindArguments::new(&queue_name, exchange_name, "bassinet.AccountBound"))
        .await
        .context("queue binding failed")?;

    let consume_args = BasicConsumeArguments::new(&queue_name, "AccountBound").auto_ack(false).finish();
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

            // 处理消息
            // {
            //     "address": "0x87e487cd6b1c7a53f91999eb3a5372ced201b614b26924ba4cc1d282a2240c07",
            //     "public_key": "dd277b01a2c6731d56354dc167ccf73e78d9b9aed0aa02c0ff3a77f3b3968e23",
            //     "success": true
            // }
            let value : Value = serde_json::from_str(json).unwrap();
            let address = value.get("address");
            let public_key = value.get("public_key");
            let success = value.get("success");
            if address.is_some() && public_key.is_some() {
                let public_key = public_key.unwrap().as_str().unwrap().to_owned();
                let wallet_address = address.unwrap().as_str().unwrap().to_owned();
                let success = success.unwrap().as_bool().unwrap();
                println!("address:{}, public_key:{}, success:{}", wallet_address, public_key, success);
                if success {
                    sui_application_service::confirm_account_bound(&public_key, wallet_address).await;
                }
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