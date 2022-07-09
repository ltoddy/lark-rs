use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};

pub struct AuthClient {
    httpclient: reqwest::Client,
}

impl AuthClient {
    pub fn new(httpclient: reqwest::Client) -> Self {
        Self { httpclient }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AccessTokenRequest {
    app_id: String,
    app_secret: String,
}

impl AccessTokenRequest {
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self { app_id, app_secret }
    }
}

struct AccessTokenResponse {
    code: i32,
    msg: String,
    tenant_access_token: Option<String>,
    expire: Option<i32>,
}

impl AuthClient {
    pub fn fetch_access_token(&self, app_id: String, app_secret: String) {
        let url = "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal";

        let request = AccessTokenRequest::new(app_id, app_secret);
        let response = self
            .httpclient
            .post(url)
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .json(&request)
            .send()
            .await
            .unwrap()
            .json::<AccessTokenResponse>()
            .await
            .unwrap();
    }
}
