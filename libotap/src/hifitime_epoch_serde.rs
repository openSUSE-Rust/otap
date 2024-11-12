// SPDX-License-Identifier: EUPL-1.2

// Copyright (C) 2023  Soc Virnyl Estela

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
