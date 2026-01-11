use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "error")]
pub struct Error {
    pub code: u16,
    pub message: String,
    #[serde(rename = "helpUrl")]
    pub help_url: Option<String>,
}

impl Error {
    pub fn into_response(self) -> super::subsonic::SubsonicResponse<Error> {
        super::subsonic::SubsonicResponse::error(self)
    }

    pub fn param_missing() -> Self {
        Self {
            code: 10,
            message: "Required parameter is missing.".to_string(),
            help_url: None,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            code: 40,
            message: "Wrong username or password.".to_string(),
            help_url: None,
        }
    }
}
