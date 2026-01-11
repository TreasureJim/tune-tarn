use serde::Serialize;

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct SubsonicResponse<T = EmptyResponse> {
    // Command result
    pub status: options::Status,
    #[serde(rename = "version")]
    pub subsonic_version: String,
    #[serde(rename = "type")]
    pub server_name: String,
    #[serde(rename = "serverVersion")]
    pub server_version: String,
    #[serde(rename = "openSubsonic")]
    pub open_subsonic: bool,
    #[serde(flatten)]
    data: T
}

#[derive(Serialize)]
pub struct EmptyResponse;

pub mod options {
    use super::*;

    #[derive(Serialize)]
    pub enum Status {
        #[serde(rename = "ok")]
        Ok,
        #[serde(rename = "failed")]
        Failed
    }
}

impl<T> SubsonicResponse<T> {
    pub fn default(data: T) -> Self {
        Self {
            status: options::Status::Ok,
            subsonic_version: crate::global::SUBSONIC_VERSION.to_string(),
            server_name: crate::global::SERVER_NAME.to_string(),
            server_version: crate::global::SERVER_VERSION.to_string(),
            // TODO: IMPORTANT!!! CHANGE WHEN COMPLIANT
            open_subsonic: false,
            data,
        }
    }

    pub fn status(mut self, status: options::Status) -> Self {
        self.status = status;
        self
    }
}

impl SubsonicResponse<super::error::Error> {
    pub fn error(e: super::error::Error) -> Self {
        Self::default(e).status(options::Status::Failed)
    }
}

impl SubsonicResponse<EmptyResponse> {
    pub fn empty_response() -> Self {
        Self::default(EmptyResponse {})
    }
}
