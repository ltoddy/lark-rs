use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct AccessTokenRequest {
    app_id: String,
    app_secret: String,
}

impl AccessTokenRequest {
    pub(crate) fn new(app_id: String, app_secret: String) -> Self {
        Self { app_id, app_secret }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct AccessTokenResponse {
    pub(crate) code: i32,
    pub(crate) msg: String,
    #[serde(flatten)]
    pub(crate) tae: Option<TokenAndExpire>,
}

#[derive(Debug, Deserialize)]
pub struct TokenAndExpire {
    pub tenant_access_token: String,
    pub expire: i32,
}
