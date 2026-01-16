use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{
    AppState,
    models::{self, api_keys::RawApiKey},
};

pub async fn add_user(State(state): State<Arc<AppState>>) -> Response {
    let user = match models::users::User::add_user(&state.pool).await {
        Ok(user) => user,
        Err(e) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    let api_key_raw = RawApiKey::generate(None);

    if let Err(e) =
        models::api_keys::ApiKey::add_api_key(&state.pool, user.id, None, &api_key_raw).await
    {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    };

    (
        StatusCode::OK,
        Json(json!({
            "id": user.id,
            "api_key": api_key_raw.to_string()
        })),
    )
        .into_response()
}
