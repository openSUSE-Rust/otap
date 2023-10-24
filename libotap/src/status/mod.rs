use crate::consts::LIST_MESSAGES_LIMIT;
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
    // TODO Severity should be an enum
    severity: String,
    scope: String,
    // TODO We should use hifitime here or chrono but oh well
    created_at: String,
}

// TODO Some methods are similar with `StatusMessage` and `StatusMessages`. This could be a possible trait impl
impl StatusMessages {
    pub fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<StatusMessages>(xml_str)
    }

    pub fn list_messages(self, limit: Option<usize>) -> Option<Vec<StatusMessage>> {
        if !self.status_message.is_empty() {
            Some(match limit {
                Some(lim) => self.status_message[..=lim].to_vec(),
                None => self.status_message[..=LIST_MESSAGES_LIMIT].to_vec(),
            })
        } else {
            None
        }
    }

    pub fn get_msg_by_id(&self, id: u64) -> Result<String, StatusError> {
        for status_message in self.status_message.iter() {
            if status_message.id == id {
                return Ok(status_message.message.clone());
            }
        }
        Err(StatusError::NonExistentId(id))
    }

    pub fn get_status_msgs_by_id(&self, id: u64) -> Result<StatusMessage, StatusError> {
        for status_message in self.status_message.iter() {
            if status_message.id == id {
                return Ok(status_message.clone());
            }
        }
        Err(StatusError::NonExistentId(id))
    }

    pub fn get_status_msgs_by_user(&self, user: &str) -> Result<Vec<StatusMessage>, StatusError> {
        let mut status_messages_from_user: Vec<StatusMessage> = Vec::new();
        for status_message in self.status_message.iter() {
            if status_message.user == user {
                status_messages_from_user.push(status_message.clone());
            }
        }

        if status_messages_from_user.is_empty() {
            Err(StatusError::NoUserFound(user.to_string()))
        } else {
            Ok(status_messages_from_user)
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatusError {
    NonExistentId(u64),
    NoUserFound(String),
}

impl StatusMessage {
    pub fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<StatusMessage>(xml_str)
    }

    pub fn get_msg_by_id(self, id: u64) -> Result<String, StatusError> {
        if self.id != id {
            return Err(StatusError::NonExistentId(id));
        };
        Ok(self.message)
    }
}
