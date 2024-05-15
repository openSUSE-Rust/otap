// SPDX-License-Identifier: EUPL-1.2

// Copyright Gordon Leung

use reqwest::Error;
use url::Url;

use crate::consts::API_URL;

#[tokio::main]
pub async fn get_services() -> Result<(), Error> {
    let url = Url::parse(&(API_URL.to_string() + "service")).expect("`API_URL` should have backslash");

    let resp = reqwest::get(url).await?.text().await?;

    Ok(println!("{}",resp))
}
