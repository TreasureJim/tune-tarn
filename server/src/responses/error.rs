use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename = "error")]
pub struct Error {
    pub code: u16,
    pub message: String,
    #[serde(rename = "helpUrl")]
    pub help_url: Option<String>
}
