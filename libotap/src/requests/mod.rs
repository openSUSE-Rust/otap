// SPDX-License-Identifier: EUPL-1.2

// Copyright (C) 2023  Soc Virnyl Estela
// Copyright (C) Gordon Leung

use reqwest::{Client, StatusCode};
use url::Url;

use crate::consts::API_URL;

pub async fn get(id: usize, username: String, password: String) -> Result<(), String> {
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
        return match status {
            StatusCode::NOT_FOUND => Err("Request does not exist".to_string()),
            StatusCode::UNAUTHORIZED => Err("Credentials not accepted".to_string()),
            err => Err(format!("Unknown error with status: {err}")),
        };
    }

    // TODO: Now that we got a appropriate response, deserialize it,
    //       then return that result for pretty printing at main.rs
    Ok(println!("{}", resp.text().await.map_err(|e| e.to_string())?))
}
