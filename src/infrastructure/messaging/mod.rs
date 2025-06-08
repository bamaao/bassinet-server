use std::env;
use tokio::time::{sleep, Duration};

pub mod account_bound_consumer;
pub mod coin_published_consumer;
pub mod nft_published_consumer;
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// /// Main program entry point. Use a structure that reflects real-world programs,
// /// e.g. multiple concurrent tasks, sharing some global state. Support for graceful
// /// shutdown, and for reconnecting when connection to a server is lost.
// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     dotenvy::dotenv()?;
//     // tracing_subscriber::fmt::init();
//     tracing_subscriber::registry()
//     .with(
//         tracing_subscriber::EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
//     ).with(tracing_subscriber::fmt::layer())
//     .init();

//     let config = Arc::new(load_config().await);

//     tokio::select!(
//         result = rabbit_manager(config.clone()) => {
//             tracing::error!("rabbit exited: {result:?}");
//         },
//         result = shutdown_monitor(config.clone()) => {
//             tracing::warn!("shutdown command received: {result:?}");
//         }
//         // We would start other tasks here too, e.g. HTTP server
//     );

//     Ok(())
// }

/// Load the application configuration.
/// Uses environment variable, but in reality it might use some other external configuration source.
pub async fn load_config() -> Config {
    sleep(Duration::from_millis(10)).await; // delay to simulate loading configuration
    Config {
        virtual_host: env::var("RABBIT_VHOST").unwrap_or("/".to_owned()),
        host: env::var("RABBIT_HOST").unwrap_or("localhost".to_owned()),
        password: env::var("RABBIT_PASSWORD").unwrap_or("guest".to_owned()),
        port: env::var("RABBIT_PORT")
            .map(|s| s.parse::<u16>().expect("can't parse RABBIT_PORT"))
            .unwrap_or(5672),
        username: env::var("RABBIT_USER").unwrap_or("guest".to_owned()),
    }
}

// async fn shutdown_monitor(cfg: Arc<Config>) -> anyhow::Result<String> {
//     // Show how tasks can share access to application config, though obviously we don't need config here right now.
//     info!(
//         "waiting for Ctrl+C.  I have access to the configuration. Rabbit host: {}",
//         cfg.host
//     );
//     tokio::signal::ctrl_c()
//         .await
//         .context("problem waiting for Ctrl+C")?;
//     info!("Received Ctrl+C signal");

//     // Spawn a task that will immediately abort if a second Ctrl+C is received while shutting down.
//     tokio::task::spawn(async {
//         match tokio::signal::ctrl_c().await {
//             Ok(()) => {
//                 warn!("Aren't you in a hurry?!");
//             }
//             Err(err) => {
//                 error!("problem waiting for Ctrl+C 2nd time: {err:?}");
//             }
//         };
//         warn!("aborting process due to 2nd Ctrl+C");
//         std::process::abort();
//     });

//     Ok("Ctrl+C".to_owned())
// }

/// Application configuration data.
pub struct Config {
    pub virtual_host: String,
    pub host: String,
    pub password: String,
    pub port: u16,
    pub username: String,
}

// #[derive(Error, Debug)]
// pub enum RabbitError {
//     #[error("RabbitMQ server connection lost: {0}")]
//     ConnectionLost(String),
// }