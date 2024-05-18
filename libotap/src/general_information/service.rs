// SPDX-License-Identifier: EUPL-1.2

// Copyright Gordon Leung

use reqwest::Client;
use url::Url;

use crate::consts::API_URL;

pub async fn get_services() -> Result<(), reqwest::Error> {
    // This should always construct a valid URL
    #[allow(clippy::unwrap_used)]
    let url = Url::parse(&(API_URL.to_string() + "service")).unwrap();

    let resp = Client::new()
        .get(url)
        .header("accept", "application/xml; charset=utf-8")
        .send()
        .await?
        .text()
        .await?;

    Ok(println!("{}", resp))
}
