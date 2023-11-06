use crate::consts::LIST_MESSAGES_LIMIT;

use std::str::FromStr;

use hifitime::Epoch;
use quick_xml::{de::from_str, DeError};
use serde::Deserialize;
use serde::Serialize;

mod hifitime_epoch_serde {
    use hifitime::efmt::consts::ISO8601;
    use hifitime::prelude::*;
    use serde::de::Error;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;
    pub fn serialize<S: Serializer>(epoch: &Epoch, serializer: S) -> Result<S::Ok, S::Error> {
        let time = format!("{}", Formatter::new(epoch.to_owned(), ISO8601));
        time.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Epoch, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        time.parse::<Epoch>().map_err(D::Error::custom)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StatusMessages {
    #[serde(rename = "@count")]
    count: u64,
    #[serde(rename = "status_message")]
    status_messages: Vec<StatusMessage>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StatusMessage {
    #[serde(rename = "@id")]
    id: u64,
    message: String,
    user: String,
    // TODO Should this be an enum?
    severity: String,
    scope: String,
    #[serde(with = "hifitime_epoch_serde")]
    created_at: Epoch,
}

// INFO Some methods are similar with `StatusMessage` and `StatusMessages`. This could be a possible trait impl

impl FromStr for StatusMessages {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<StatusMessages>(xml_str)
    }
}

impl FromStr for StatusMessage {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<StatusMessage>(xml_str)
    }
}

impl StatusMessages {

    /// Returns the number of messages received
    pub fn count(self) -> u64 {
        self.count 
    }

    /// List messages based on a limit. Default length is 1000 or defined in `LIST_MESSAGES_LIMIT`
    pub fn list_messages(self, limit: Option<usize>) -> Option<Vec<StatusMessage>> {
        if !self.status_messages.is_empty() {
            Some(match limit {
                Some(lim) => self.status_messages[..=lim].to_vec(),
                None => self.status_messages[..=LIST_MESSAGES_LIMIT].to_vec(),
            })
        } else {
            None
        }
    }
    
    /// Get messages id.
     pub fn get_message_by_id(&self, id: u64) -> Result<String, StatusError> {
         for status_message in self.status_messages.iter() {
             if status_message.id == id {
                 return Ok(status_message.message.clone());
             }
         }
         Err(StatusError::NonExistentId(id))
     }

    /// Get status message object by id
    pub fn get_status_message_by_id(&self, id: u64) -> Result<StatusMessage, StatusError> {
        for status_message in self.status_messages.iter() {
            if status_message.id == id {
                return Ok(status_message.clone());
            }
        }
        Err(StatusError::NonExistentId(id))
    }

    /// Get status message object by user
    pub fn get_status_message_by_user(
        &self,
        user: &str,
    ) -> Result<Vec<StatusMessage>, StatusError> {
        let mut status_messages_from_user: Vec<StatusMessage> = Vec::new();
        for status_message in self.status_messages.iter() {
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
