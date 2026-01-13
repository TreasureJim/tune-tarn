mod authentication;
mod endpoints;
mod global;
mod middleware;
mod models;
mod responses;

use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, sync::Arc};

use axum::Router;
use sqlx::Postgres;
use tower_http::trace::TraceLayer;
use tracing::{Span, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    pub pool: sqlx::Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=info,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    log::info!("Connecting to database...");
    let connection_str =
        std::env::var("DATABASE_URL").expect("Expected to find env var DATABASE_URL");
    let pool = sqlx::PgPool::connect_lazy(&connection_str).expect("Failed to connect to database");

    let app_state = Arc::new(AppState { pool });

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
    log::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router(app_state)).await.unwrap();
}

fn router(state: Arc<AppState>) -> axum::extract::connect_info::IntoMakeServiceWithConnectInfo<Router, SocketAddr> {
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::extract::Request| {
            let ip = request
                .extensions()
                .get::<axum::extract::ConnectInfo<SocketAddr>>()
                .map(|ci| ci.ip().to_string())
                .unwrap_or("Unknown IP".to_string());

            info_span!(
                "request",
                request_ip = ip,
                method = %request.method(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        })
        .on_response(
            |response: &axum::response::Response, latency: std::time::Duration, span: &Span| {
                // Log at your desired level with the span context
                tracing::info!(
                    parent: span,
                    status = %response.status(),
                    latency = ?latency,
                    "finished processing request"
                );
            },
        );

    Router::<Arc<AppState>>::new()
        .with_state(state.clone())
        .nest("/rest", endpoints::rest::get_router(state.clone()))
        .layer(tracing_layer)
        .into_make_service_with_connect_info::<SocketAddr>()
}
