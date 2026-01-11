use axum::Json;
use serde_json::Value;

use crate::responses::subsonic::SubsonicResponse;

#[axum::debug_handler]
pub async fn ping() -> Json<Value> {
    Json(serde_json::json!(SubsonicResponse::empty_response()))
}
