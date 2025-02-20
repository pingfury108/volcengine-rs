use chrono::Utc;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

type HmacSha256 = Hmac<Sha256>;

fn sign(key: &[u8], msg: &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can be created");
    mac.update(msg.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

fn get_signature_key(secret_key: &str, date_stamp: &str, region: &str, service: &str) -> Vec<u8> {
    let k_date = sign(secret_key.as_bytes(), date_stamp);
    let k_region = sign(&k_date, region);
    let k_service = sign(&k_region, service);
    sign(&k_service, "request")
}

fn format_query(parameters: &BTreeMap<&str, &str>) -> String {
    parameters
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect::<Vec<_>>()
        .join("&")
}

#[allow(clippy::too_many_arguments)]
fn signature_v4(
    access_key: &str,
    secret_key: &str,
    host: &str,
    region: &str,
    service: &str,
    method: &str,
    req_query: &str,
    req_body: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let now = Utc::now();
    let current_date = now.format("%Y%m%dT%H%M%SZ").to_string();
    let datestamp = now.format("%Y%m%d").to_string();

    let payload_hash = {
        let mut hasher = Sha256::new();
        hasher.update(req_body.as_bytes());
        hex::encode(hasher.finalize())
    };

    let content_type = "application/json";
    let signed_headers = "content-type;host;x-content-sha256;x-date";

    let canonical_headers = format!(
        "content-type:{}\nhost:{}\nx-content-sha256:{}\nx-date:{}\n",
        content_type, host, payload_hash, current_date
    );

    let canonical_request = format!(
        "{}\n/\n{}\n{}\n{}\n{}",
        method, req_query, canonical_headers, signed_headers, payload_hash
    );

    let algorithm = "HMAC-SHA256";
    let credential_scope = format!("{}/{}/{}/request", datestamp, region, service);

    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        algorithm,
        current_date,
        credential_scope,
        hex::encode(Sha256::digest(canonical_request.as_bytes()))
    );

    let signing_key = get_signature_key(secret_key, &datestamp, region, service);
    let signature = hex::encode(sign(&signing_key, &string_to_sign));

    let authorization_header = format!(
        "{} Credential={}/{}, SignedHeaders={}, Signature={}",
        algorithm, access_key, credential_scope, signed_headers, signature
    );

    Ok((authorization_header, current_date))
}

fn get_host(endpoint: &str) -> Result<String, String> {
    reqwest::Url::parse(endpoint)
        .map_err(|e| e.to_string())?
        .host_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid endpoint".into())
}

#[allow(clippy::too_many_arguments)]
pub async fn send_request<T: DeserializeOwned>(
    access_key: &str,
    secret_key: &str,
    endpoint: &str,
    region: &str,
    service: &str,
    method: &str,
    content_type: &str,
    query_params: BTreeMap<&str, &str>,
    body_params: serde_json::Value,
) -> Result<T, Box<dyn std::error::Error>> {
    let client = Client::new();
    let formatted_query = format_query(&query_params);
    let formatted_body = body_params.to_string();

    let host = get_host(endpoint)?;

    let (authorization_header, current_date) = signature_v4(
        access_key,
        secret_key,
        &host,
        region,
        service,
        method,
        &formatted_query,
        &formatted_body,
    )?;

    let payload_hash = hex::encode(Sha256::digest(formatted_body.as_bytes()));

    let request_url = format!("{}?{}", endpoint, formatted_query);

    let response = client
        .request(method.parse()?, request_url)
        .header("X-Date", current_date)
        .header("Authorization", authorization_header)
        .header("X-Content-Sha256", payload_hash)
        .header("Content-Type", content_type)
        .body(formatted_body)
        .send()
        .await?;

    let data = response.json::<T>().await?;

    Ok(data)
}
