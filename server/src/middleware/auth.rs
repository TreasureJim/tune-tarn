use axum::{
    extract::{Query, Request}, http::{HeaderMap, StatusCode}, middleware::Next, response::{IntoResponse, Response}, Json, RequestExt
};
use serde::Deserialize;
use serde_json::json;

use crate::responses;

#[derive(Deserialize)]
struct AuthParams {
    #[serde(rename="apiKey")]
    api_key: String
}

pub async fn auth(request: Request, next: Next) -> Response {
    let Ok(auth_params) = Query::<AuthParams>::try_from_uri(request.uri()) else {
        let mut e_res = responses::error::Error::param_missing();
        e_res.message.push_str(" Provide 'apiKey'.");
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!(e_res)),
        ).into_response();
    };

    if auth_params.api_key != "1" {
        todo!("Implement database with API keys");
        // responses::error::Error::unauthorized()
    }

    next.run(request).await
}
