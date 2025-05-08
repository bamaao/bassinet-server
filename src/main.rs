
use std::{env, /*net::SocketAddr,*/ path::{Path, /*PathBuf*/}, /*str::FromStr,*/ sync::Arc};

use axum::{http::{Method, StatusCode}, response::IntoResponse, routing::{get, post}, Router};
use config::{Config, File};
// use futures::StreamExt;
use interface::rest::{file_api, logon_api::{sign_in, sign_up}, my_collection_api::{self}, public_collection_api, request_id};
// use tokio_rustls_acme::{axum::AxumAcceptor, caches::DirCache, AcmeConfig};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// use rustls::{crypto::CryptoProvider, ServerConfig as RustlsServerConfig};

mod infrastructure;

mod domain;

mod interface;

mod application;

mod utils;


#[derive(Clone)]
pub struct ServerConfig {
    pub assets_path: String,
    pub assets_addr: String,
    pub assets_http_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    dotenvy::dotenv()?;
    // for (key, value) in env::vars() {
    //     println!("{key}: {value}");
    // }
    // initialize tracing
    // tracing_subscriber::fmt::init();
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
    ).with(tracing_subscriber::fmt::layer())
    .init();

    // rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");

    // let path_buf = PathBuf::from_str("./conf").unwrap();
    // let mut state = AcmeConfig::new(["bassinet.app"])
    // .contact(["mailto:cmmymtzxl@126.com"])
    // .cache_option(Some(DirCache::new(path_buf)))
    // .directory_lets_encrypt(false)
    // .state();

    // let rustls_config = RustlsServerConfig::builder()
    // .with_no_client_auth()
    // .with_cert_resolver(state.resolver());

    // let acceptor = state.axum_acceptor(Arc::new(rustls_config));

    // tokio::spawn(async move{
    //     loop {
    //         match state.next().await.unwrap() {
    //             Ok(ok) => tracing::info!("event: {:?}", ok),
    //             Err(err) => tracing::error!("error: {:?}", err),
    //         }
    //     }
    // });

    // 读取配置
    let settings = Config::builder()
    .add_source(File::from(Path::new("conf/default.toml")))
    .build()
    .unwrap();

    // build our application with a route
    // web服务地址
    let web_addr = settings.get_string("web_addr").unwrap();

    // let addr = SocketAddr::from_str(&web_addr).unwrap();

    // assets服务地址
    let assets_addr = settings.get_string("assets_addr").unwrap();

    let assets_path = settings.get_string("assets_path").unwrap();

    let settings = Arc::new(settings);
    tokio::join!(
        // web service
        http_web_serve(webservice_router(settings), &web_addr),
        // https_web_serve(webservice_router(settings), addr, acceptor),
        // static assets
        assets_serve(using_serve_dir(&assets_path), &assets_addr),
    );

    Ok(())

}

fn webservice_router(settings: Arc<Config>) -> Router{
    let server_config = Arc::new(ServerConfig {
        assets_path: settings.get_string("assets_path").unwrap(),
        assets_addr: settings.get_string("assets_addr").unwrap(),
        assets_http_addr: settings.get_string("assets_http_addr").unwrap(),
    });

    // let cors = CorsLayer::new()
    // .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
    // .allow_origin(Any)
    // .allow_credentials(false);
    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS, Method::PUT, Method::DELETE])
    .allow_origin(Any)
    .allow_headers(Any);

    // let cors = CorsLayer::new()
    // .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    // .allow_methods([Method::GET, Method::POST]);

    let app = Router::new()
    .route("/upload", post(file_api::upload_file))
    .route("/signup", post(sign_up))
    .route("/signin", post(sign_in))
    .route("/my_collections", post(my_collection_api::create_collection).get(my_collection_api::get_my_collections))
    .route("/my_collections/{collection_id}", get(my_collection_api::get_my_collection_info_by_id))
    .route("/simple_collections", get(my_collection_api::get_simple_collections))
    .route("/author/{author_id}/collections", get(public_collection_api::get_author_collections))
    .route("/collections", get(public_collection_api::search_collections))
    .route("/collections/{collection_id}", get(public_collection_api::get_collection_info_by_id))
    .route("/articles", post(my_collection_api::create_article))
    .route("/articles/{article_id}", get(public_collection_api::get_article_by_id))
    .route("/request_id", get(request_id))
    // .layer(tower_http::cors::CorsLayer::permissive())
    .layer(cors)
    .with_state(server_config);

    // add a fallback service for handling routes to unknown paths
    let app = app.fallback(handler_404);
    app
}

fn using_serve_dir(assets_path: &str) -> Router {
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/assets", ServeDir::new(assets_path))
}

// async fn https_web_serve(app: Router, addr: SocketAddr, acceptor: AxumAcceptor) {
//     axum_server::bind(addr)
//         .acceptor(acceptor)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

async fn http_web_serve(app: Router, addr: &str) {
    // let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn assets_serve(app: Router, addr: &str) {
    // let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}