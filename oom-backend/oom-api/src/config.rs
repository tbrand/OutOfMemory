#[derive(Deserialize, Clone)]
pub struct Config {
    pub github: Github,
}

#[derive(Deserialize, Clone)]
pub struct Github {
    pub client_id: String,
    pub client_secret: String,
    pub oauth_access_token: String,
    pub oauth_authorize: String,
    pub api_endpoint: String,
}
