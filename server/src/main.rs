mod responses;
mod endpoints;
mod global;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::Router;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("tune_tarn=info,axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    let router = Router::new()
        // .nest_service("/media", ServeDir::new("media"))
        .nest("/rest", endpoints::rest::get_router())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
    log::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
