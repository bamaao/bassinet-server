// use std::time::Duration;

// use sea_orm::{ConnectOptions, Database, DatabaseConnection};

// #[derive(Debug)]
// pub struct DatabaseService {
//     pub connection: DatabaseConnection,
// }

// impl DatabaseService {
//     pub async fn init() -> Self {
//         let mut connection_options = ConnectOptions::new(std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));
//         connection_options
//         . max_connections(100)
//         .min_connections(5)
//         .connect_timeout(Duration::from_secs(8))
//         .acquire_timeout(Duration::from_secs(8))
//         .idle_timeout(Duration::from_secs(8))
//         .max_lifetime(Duration::from_secs(8))
//         .sqlx_logging(false);

//         let connection = Database::connect(connection_options).await.expect("Can't connect to database");

//         Self {connection}
//     }
// }