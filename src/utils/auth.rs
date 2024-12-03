use crate::apis::configuration::Configuration;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use sha1::{Digest, Sha1};
use time::OffsetDateTime;

pub async fn add_auth_headers(
    mut req_builder: reqwest::RequestBuilder,
    configuration: &Configuration,
) -> reqwest::RequestBuilder {
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.header("X-Auth-Key", configuration.api_key.clone());

    let current_epoch = OffsetDateTime::now_utc().unix_timestamp().to_string();
    req_builder = req_builder.header("X-Auth-Date", current_epoch.clone());

    let auth_header = format!(
        "{}{}{}",
        configuration.api_key.clone(),
        configuration.api_secret.clone(),
        current_epoch.clone()
    );
    let mut hasher = Sha1::new();
    hasher.update(auth_header.as_bytes());
    let auth_header = format!("{:x}", hasher.finalize());
    req_builder = req_builder.header("Authorization", auth_header.clone());

    req_builder
}
