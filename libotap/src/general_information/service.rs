// SPDX-License-Identifier: EUPL-1.2

// Copyright Gordon Leung

use reqwest::Error;
use url::Url;

use crate::consts::API_URL;


#[tokio::main]
pub async fn get_services() -> Result<(), Error> {
    // This should always construct a valid URL
    #[allow(clippy::unwrap_used)]
    let url = Url::parse(&(API_URL.to_string() + "service")).unwrap();

    let resp = reqwest::get(url).await?.text().await?;

    Ok(println!("{}",resp))
}
