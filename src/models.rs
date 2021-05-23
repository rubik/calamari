pub trait BaseClient {}

#[derive(Debug, Clone)]
pub(crate) enum Endpoint {
    Public(&'static str),
    Private(&'static str, ApiCredentials),
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match *self {
            Endpoint::Public(e) => format!("public/{}", e),
            Endpoint::Private(e, _) => format!("private/{}", e),
        }
    }
}

/// A type holding the base parameters for the API: the URL and the API version.
/// The Default implementation sets them to `https://api.kraken.com` and `0`
/// respectively.
#[derive(Debug, Clone, PartialEq)]
pub struct ApiParams {
    pub(crate) url: String,
    pub(crate) version: String,
}

impl ApiParams {
    pub fn new(url: String, version: String) -> Self {
        Self { url, version }
    }
}

impl Default for ApiParams {
    fn default() -> Self {
        Self {
            url: "https://api.kraken.com".into(),
            version: "0".into(),
        }
    }
}

/// A type holding the API credentials: the API key and secret.
#[derive(Debug, Clone)]
pub struct ApiCredentials {
    pub(crate) api_key: String,
    pub(crate) api_secret: String,
}

impl ApiCredentials {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_key,
            api_secret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_string() {
        let cred = ApiCredentials::new("key".into(), "secret".into());
        assert_eq!(
            Endpoint::Public("Ticker").to_string(),
            "public/Ticker".to_string()
        );
        assert_eq!(
            Endpoint::Private("Balance", cred).to_string(),
            "private/Balance".to_string()
        );
    }

    #[test]
    fn test_api_params_default() {
        assert_eq!(
            ApiParams::default(),
            ApiParams::new("https://api.kraken.com".into(), "0".into())
        );
    }
}
