use quick_xml::de::from_str;
use quick_xml::DeError;
use serde::Deserialize;
use serde::Serialize;

use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StatusReports {
    #[serde(rename = "check")]
    checks: Option<Vec<CheckName>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct CheckName {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@required")]
    required: bool,
    state: String,
    short_description: String,
    url: String,
}

impl FromStr for StatusReports {
    type Err = DeError;

    fn from_str(xml_str: &str) -> Result<Self, Self::Err> {
        from_str::<StatusReports>(xml_str)
    }
}

impl FromStr for CheckName {
    type Err = DeError;

    fn from_str(xml_str: &str) -> Result<Self, Self::Err> {
        from_str::<CheckName>(xml_str)
    }
}

impl StatusReports {
    pub fn len(self) -> u64 {
        if let Some(checks) = self.checks {
            checks.len() as u64
        } else {
            0u64
        }
    }

    pub fn get_status_reports_by_name(self, name: &str) -> Option<Vec<CheckName>> {
        if let Some(checks) = self.checks {
            Some(
                checks
                    .iter()
                    .filter(|check| check.name == name)
                    .map(|check| check.to_owned())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn get_status_reports_by_required(self, required: bool) -> Option<Vec<CheckName>> {
        if let Some(checks) = self.checks {
            Some(
                checks
                    .iter()
                    .filter(|check| check.required == required)
                    .map(|check| check.to_owned())
                    .collect(),
            )
        } else {
            None
        }
    }
}
