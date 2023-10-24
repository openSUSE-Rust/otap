use quick_xml::{de::from_str, DeError};
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StatusMessages {
    #[serde(rename = "@count")]
    count: u64,
    status_message: Vec<StatusMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StatusMessage {
    #[serde(rename = "@id")]
    id: u64,
    message: String,
    // TODO This parameter may be shared across so this should be a struct?
    user: String,
    severity: String,
    scope: String,
    // TODO We should use hifitime here or chrono but oh well
    created_at: String,
}

impl StatusMessages {
    pub fn from_str(data: &str) -> Result<Self, DeError> {
        from_str::<StatusMessages>(data)
    }
}
