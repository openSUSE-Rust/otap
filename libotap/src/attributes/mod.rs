use std::str::FromStr;

use quick_xml::{de::from_str, DeError};

use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Directory {
    #[serde(rename = "entry")]
    entries: Option<Vec<Entry>>,
}

impl Directory {
    pub fn get_entries(self) -> Option<Vec<Entry>> {
        match self.entries {
            Some(entries) => {
                if entries.is_empty() {
                    return None;
                };
                Some(entries)
            }
            None => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    #[serde(rename = "@name")]
    name: Option<String>,
}

impl FromStr for Directory {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<Directory>(xml_str)
    }
}

impl FromStr for Entry {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<Entry>(xml_str)
    }
}

// TODO Finish up meta
