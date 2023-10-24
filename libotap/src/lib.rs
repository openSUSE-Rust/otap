// SPDX-License-Identifier: MPL-2.0

// Copyright (C) 2023  Soc Virnyl Estela

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub mod announcements;
pub mod attributes;
pub mod build;
pub mod comments;
pub mod configuration;
pub mod distribution;
pub mod general_information;
pub mod groups;
pub mod issue_trackers;
pub mod notifications;
pub mod person;
pub mod published;
pub mod requests;
pub mod search;
pub mod sources;
pub mod staging;
pub mod status;

pub mod consts;
pub use crate::consts::API_HEADER;
pub use crate::consts::API_URL;
