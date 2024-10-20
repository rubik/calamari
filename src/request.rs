use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Result,
};
use sha2::{Digest, Sha256, Sha512};
use std::string::ToString;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{ApiParams, Endpoint};

type HmacSha512 = Hmac<Sha512>;

pub(crate) async fn api_request(
    http: &Client,
    api_params: &ApiParams,
    endpoint: Endpoint,
    url_encoded_body: &str,
) -> Result<String> {
    let api_path =
        format!("/{}/{}", api_params.version, endpoint.to_string(),);
    let mut api_endpoint = format!("{}{}", api_params.url, api_path);
    let api_response = match endpoint {
        Endpoint::Public(_) => {
            if !url_encoded_body.is_empty() {
                api_endpoint = api_endpoint + "?" + url_encoded_body;
            }
            http.get(&api_endpoint).send().await
        }
        Endpoint::Private(_, credentials) => {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let payload_nonce = format!("nonce={}", &nonce.to_string());
            let payload_body = if !url_encoded_body.is_empty() {
                format!("{}&{}", payload_nonce, url_encoded_body)
            } else {
                payload_nonce
            };
            let signature = get_signature(
                api_path,
                payload_body.to_owned(),
                nonce.to_string(),
                credentials.api_secret,
            );
            http.post(&api_endpoint)
                .headers(get_headers(&credentials.api_key, &signature))
                .body(payload_body)
                .send()
                .await
        }
    };
    match api_response {
        Ok(result) => result.text().await,
        Err(error) => Err(error),
    }
}

fn get_signature(
    api_path: String,
    url_encoded_body: String,
    nonce: String,
    api_secret: String,
) -> String {
    let hash_digest =
        Sha256::digest(format!("{}{}", nonce, url_encoded_body).as_bytes());
    let private_key =
        base64::decode(&api_secret).expect("invalid private key");
    let mut mac = HmacSha512::new_from_slice(&private_key).unwrap();
    let mut hmac_data = api_path.into_bytes();
    hmac_data.append(&mut hash_digest.to_vec());
    mac.update(&hmac_data);
    base64::encode(mac.finalize().into_bytes())
}

fn get_headers(api_key: &str, signature: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("API-Key", HeaderValue::from_str(api_key).unwrap());
    headers.insert("API-Sign", HeaderValue::from_str(signature).unwrap());
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_signature_invalid_private_key() {
        let api_path = "public/Ticker".to_string();
        let body = "pair=XBTUSD".to_string();
        let nonce = "1693294932349".to_string();
        let secret = "secret".to_string();
        get_signature(api_path, body, nonce, secret);
    }

    #[test]
    fn test_signature() {
        let api_path = "private/TradeBalance".to_string();
        let body = "asset=ZUSD".to_string();
        let nonce = "1693294932349".to_string();
        let secret = "c2VjcmV0".to_string();
        let signature = "VYpAzfn/HF9VfX1vYtWhctBx9Q0mIeVLXPpEPyDOHS6oPZXGe9NMSdVwShCYLI5IngUgUlmpFmRqol7++CGPBw==".to_string();
        assert_eq!(get_signature(api_path, body, nonce, secret), signature);
    }
}
