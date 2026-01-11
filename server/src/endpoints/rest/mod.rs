use axum::{routing::get, Router};

mod ping;

pub fn get_router() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
}
