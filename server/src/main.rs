mod endpoints;
mod global;
mod middleware;
mod responses;
mod authentication;
mod models;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{Router, response::IntoResponse, routing::get};
use sqlx::postgres::PgPoolOptions;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{Span, info_span};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
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

    let router = Router::new()
        .nest("/rest", endpoints::rest::get_router())
        .layer(tracing_layer)
        .into_make_service_with_connect_info::<SocketAddr>();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
    log::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
