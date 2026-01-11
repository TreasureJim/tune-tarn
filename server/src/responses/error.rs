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
    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = message.to_string();
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.help_url = Some(url.to_string());
        self
    }

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

    pub fn unsupported_token_authentication() -> Self {
        Self {
            code: 41,
            message: "Token authentication not supported for LDAP users.".to_string(),
            help_url: None,
        }
    }

    pub fn unsupported_authentication() -> Self {
        Self {
            code: 42,
            message: "Provided authentication mechanism not supported. Only 'apiKey' is supported.".to_string(),
            help_url: None,
        }
    }
}
