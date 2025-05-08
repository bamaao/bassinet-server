use std::{sync::Arc, time::Duration};

use once_cell::sync::Lazy;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::{runtime::Handle, task};

pub static DB: Lazy<Arc<DatabaseConnection>> = Lazy::new(|| {
    task::block_in_place(move || {
        Handle::current().block_on(async {
            let mut connection_options = ConnectOptions::new(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

            connection_options
            .max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

            let connection = Database::connect(connection_options).await.expect("Can't connect to database");
            Arc::new(connection)
        })
    })
});

pub fn get_db() -> Arc<DatabaseConnection> {
    DB.clone()
}