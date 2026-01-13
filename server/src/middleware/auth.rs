use std::sync::Arc;

use axum::{
    extract::{Query, Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}, Json
};
use serde::Deserialize;
use serde_json::json;

use crate::{models::api_keys::{ApiKeyError, RawApiKey}, responses::SubsonicError, AppState};

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

pub async fn auth(State(state): State<Arc<AppState>>, mut request: Request, next: Next) -> Response {
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

    let api_key = match RawApiKey::parse(&auth_params.api_key) {
        Ok(api_key) => api_key,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                SubsonicError::generic(e.to_string()).into_json(),
            )
                .into_response();
        }
    };

    let user = match api_key.get_user(&state.pool).await {
        Ok(user) => user,

        Err(ApiKeyError::ParseError(e)) => {

            return (
                StatusCode::BAD_REQUEST,
                SubsonicError::generic(e.to_string()).into_json(),
            )
                .into_response();
        }

        Err(ApiKeyError::Database(e)) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                SubsonicError::generic("Database error").into_json(),
            ).into_response();
        }

        Err(ApiKeyError::NotFound) => {
            return (
                StatusCode::UNAUTHORIZED,
                SubsonicError::invalid_api_key().into_json(),
            ).into_response();
        }
    };

    request.extensions_mut().insert(user);

    next.run(request).await
}
