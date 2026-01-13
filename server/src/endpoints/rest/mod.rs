use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use crate::{middleware::auth::auth, AppState};

mod ping;

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(ping::ping))
        .route_layer(middleware::from_fn_with_state(state, auth))
}
