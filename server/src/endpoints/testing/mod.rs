use axum::{routing::post, Router};
use std::sync::Arc;

use crate::AppState;

pub mod add_user;

pub fn get_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/add_user", post(add_user::add_user))
}
