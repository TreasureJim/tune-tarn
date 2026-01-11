use axum::{
    Json, extract::{Query, Request},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;

use crate::responses::SubsonicError;

#[derive(Deserialize, Debug)]
struct AuthParams {
    #[serde(rename = "apiKey")]
    api_key: String,
    #[serde(rename = "p")]
    password: Option<String>,
    #[serde(rename = "t")]
    token: Option<String>,
    #[serde(rename = "s")]
    salt: Option<String>,
}

pub async fn auth(request: Request, next: Next) -> Response {
    let Ok(auth_params) = Query::<AuthParams>::try_from_uri(request.uri()) else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!(
                SubsonicError::param_missing().message("Missing 'apiKey' parameter")
            )),
        )
            .into_response();
    };

    if auth_params.password.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(SubsonicError::unsupported_authentication())),
        )
            .into_response();
    }

    if auth_params.token.is_some() || auth_params.salt.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!(SubsonicError::unsupported_token_authentication())),
        )
            .into_response();
    }

    if auth_params.api_key != "1" {
        todo!("Implement database with API keys");
        // responses::error::Error::unauthorized()
    }

    next.run(request).await
}
