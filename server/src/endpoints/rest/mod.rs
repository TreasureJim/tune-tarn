use axum::{middleware, routing::get, Router};
use tower::ServiceBuilder;

use crate::middleware::auth::auth;

mod ping;

pub fn get_router() -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route_layer(middleware::from_fn(auth))
}
