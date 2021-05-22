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

#[derive(Debug, Clone, PartialEq)]
pub struct ApiParams {
    pub(crate) url: &'static str,
    pub(crate) version: &'static str,
}

impl Default for ApiParams {
    fn default() -> Self {
        Self {
            url: "https://api.kraken.com",
            version: "0",
        }
    }
}

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
            ApiParams {
                url: "https://api.kraken.com",
                version: "0"
            }
        );
    }
}
