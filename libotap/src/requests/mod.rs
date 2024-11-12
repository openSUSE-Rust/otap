// SPDX-License-Identifier: EUPL-1.2

// Copyright (C) 2023  Soc Virnyl Estela
// Copyright (C) Gordon Leung

use std::ops::Deref;

use crate::hifitime_epoch_serde;
use hifitime::Epoch;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use url::Url;

use crate::consts::API_URL;

/// https://api.opensuse.org/apidocs/#/Requests/get_request__id_
#[derive(Deserialize, Debug, Clone)]
pub struct Request {
    #[serde(rename = "@id")]
    pub id: usize,
    #[serde(rename = "@creator")]
    pub creator: String,
    pub action: RequestAction,
    pub review: Option<Vec<RequestReview>>,
    pub history: Option<Vec<RequestHistory>>,
    pub description: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestAction {
    #[serde(rename = "@type")]
    pub r#type: ActionType,
    pub source: RequestActionSource,
    pub target: RequestActionTarget,
    pub options: Option<RequestActionOptions>,
}
/// https://openbuildservice.org/help/manuals/obs-user-guide/cha-obs-request-and-review-system#id-1.2.10.9.5.5
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Submit,
    Release,
    Delete,
    AddRole,
    SetBugowner,
    ChangeDevel,
    MaintenanceIncident,
    MaintenanceRelease,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestActionSource {
    #[serde(rename = "@project")]
    pub project: String,
    #[serde(rename = "@package")]
    pub package: String,
    /// Older requests such as https://build.opensuse.org/request/show/2000 used a String as rev
    #[serde(rename = "@rev")]
    pub rev: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestActionTarget {
    #[serde(rename = "@project")]
    pub project: String,
    #[serde(rename = "@package")]
    pub package: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestActionOptions {
    pub sourceupdate: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestState {
    #[serde(rename = "@name")]
    pub name: State,
    #[serde(rename = "@who")]
    pub who: String,
    #[serde(with = "hifitime_epoch_serde", rename = "@when")]
    pub when: Epoch,
    pub comment: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestReview {
    #[serde(rename = "@state")]
    pub state: State,
    #[serde(with = "hifitime_epoch_serde", rename = "@when")]
    pub when: Epoch,
    #[serde(rename = "@who")]
    pub who: String,
    #[serde(rename = "@by_user")]
    pub by_user: Option<String>,
    #[serde(rename = "@by_group")]
    pub by_group: Option<String>,
    #[serde(rename = "@by_project")]
    pub by_project: Option<String>,
    #[serde(rename = "@by_package")]
    pub by_package: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RequestHistory {
    #[serde(rename = "@who")]
    pub who: String,
    #[serde(with = "hifitime_epoch_serde", rename = "@when")]
    pub when: Epoch,
    pub description: String,
    pub comment: String,
}
/// https://openbuildservice.org/help/manuals/obs-user-guide/cha-obs-request-and-review-system#id-1.2.10.9.5.6
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum State {
    New,
    Accepted,
    Declined,
    Revoked,
    Superseded,
    Review,
}

pub async fn get(id: usize, username: String, password: String) -> Result<Request, String> {
    // This should always construct a valid URL
    #[allow(clippy::unwrap_used)]
    let url = Url::parse(&(API_URL.to_string() + "request/" + id.to_string().as_str())).unwrap();

    let resp = Client::new()
        .get(url)
        .header("accept", "application/xml; charset=utf-8")
        .basic_auth(username, Some(password))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = resp.status();
    if status.is_client_error() || status.is_server_error() {
        match status {
            StatusCode::NOT_FOUND => Err("Request does not exist".to_string()),
            StatusCode::UNAUTHORIZED => Err("Credentials not accepted".to_string()),
            err => Err(format!("Unknown error with status: {err}")),
        }
    } else {
        // Convert the response into &[u8], then read it into a Request
        quick_xml::de::from_reader(resp.bytes().await.map_err(|e| e.to_string())?.deref())
            .map_err(|e| e.to_string())
    }
}
