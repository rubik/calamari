#[derive(Debug, Clone)]
pub enum Endpoint {
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
    pub url: &'static str,
    pub version: &'static str,
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
    pub api_key: String,
    pub api_secret: String,
}
