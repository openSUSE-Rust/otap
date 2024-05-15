// SPDX-License-Identifier: EUPL-1.2

// Copyright (C) 2023  Soc Virnyl Estela

use std::str::FromStr;

use quick_xml::{de::from_str, DeError};
use serde::Deserialize;
use serde::Serialize;

// TODO
// 1. maxmtime and time should be converted into a valid timestamp but it will be string for now.
// 2. I wonder which of this are `Option`al hmm

impl FromStr for Packages {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<Packages>(xml_str)
    }
}

impl FromStr for Package {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<Package>(xml_str)
    }
}

impl FromStr for Failure {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<Failure>(xml_str)
    }
}

impl FromStr for Link {
    type Err = DeError;
    fn from_str(xml_str: &str) -> Result<Self, DeError> {
        from_str::<Link>(xml_str)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Packages {
    #[serde(rename = "package")]
    packages: Vec<Package>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Package {
    #[serde(rename = "@changesmd5")]
    changesmd5: Option<String>,
    #[serde(rename = "failure")]
    failures: Option<Vec<Failure>>,
    #[serde(rename = "link")]
    links: Option<Vec<Link>>,
    #[serde(rename = "@maxmtime")]
    maxmtime: Option<String>,
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@project")]
    project: String,
    #[serde(rename = "@release")]
    release: String,
    #[serde(rename = "@srcmd5")]
    srcmd5: String,
    #[serde(rename = "@verifymd5")]
    verifymd5: Option<String>,
    #[serde(rename = "@version")]
    version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Failure {
    #[serde(rename = "@repo")]
    repo: String,
    #[serde(rename = "@srcmd5")]
    srcmd5: String,
    #[serde(rename = "@time")]
    time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Link {
    #[serde(rename = "@project")]
    project: String,
    #[serde(rename = "@package")]
    package: String,
}
