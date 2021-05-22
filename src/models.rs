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

#[derive(Debug, Clone)]
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
        Self { api_key, api_secret }
    }
}
