use anyhow::{anyhow, Ok, Result};
use reqwest::{header::HeaderMap, Response};
use serde::Deserialize;

const API_HOST: &str = "https://gpcfgsetupapidev.azurewebsites.net";
const API_KEY: &str = "XmrYeFSG2ki0dralOJCPrw==";

// REGISTER_ENDPOINT="${API_HOST}/dps/register?macAddress=${MAC}&processorArchitecture=${ARCH}&imageVersion=${IMG_VERSION}"
// 	curl \
// 		-s \
// 		-X POST \
// 		-H "x-key: ${API_KEY}" \
// 		-H "Content-Length: 0" \
// 		-w "\n%{http_code}" \
// 		"${REGISTER_ENDPOINT}"

pub async fn register(
    mac_address: &str,
    architecture: &str,
    image_version: &str,
) -> Result<String> {
    let url = format!(
        "{0}/dps/register?macAddress={1}&processorArchitecture={2}&imageVersion={3}",
        API_HOST, mac_address, architecture, image_version
    );

    let mut headers = HeaderMap::new();
    headers.insert("x-key", API_KEY.parse().unwrap());

    let json: String = reqwest::Client::new()
        .post(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let register_result: RegisterResult = serde_json::from_str(json.as_str())?;
    match register_result.status {
        200 | 800 => Ok(register_result.status_message.to_string()),
        _ => Err(anyhow!(register_result.status_message.to_string())),
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterResult<'a> {
    status: u32,
    status_message: &'a str,
    device: Option<RegisterDevice<'a>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegisterDevice<'a> {
    device_id: &'a str,
    primary_key: &'a str,
    hostname: &'a str,
    cryptography_key: &'a str,
}
