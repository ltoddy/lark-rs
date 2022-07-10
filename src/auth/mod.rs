pub mod app_access_token;

use reqwest::header::CONTENT_TYPE;

use self::app_access_token::{AccessTokenRequest, AccessTokenResponse, TokenAndExpire};
use crate::error::Error;
use crate::result::Result;

pub struct AuthClient {
    httpclient: reqwest::Client,
}

impl AuthClient {
    pub fn new(httpclient: reqwest::Client) -> Self {
        Self { httpclient }
    }
}

impl AuthClient {
    pub async fn fetch_access_token(&self, app_id: String, app_secret: String) -> Result<TokenAndExpire> {
        let url = "https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal";

        let request = AccessTokenRequest::new(app_id, app_secret);
        let response = self
            .httpclient
            .post(url)
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .json(&request)
            .send()
            .await?
            .json::<AccessTokenResponse>()
            .await?;

        if let Some(tae) = response.tae {
            return Ok(tae);
        }

        return Err(Error::LarkBackend(response.code, response.msg));
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[tokio::test]
    pub async fn fetch_access_token() -> Result<()> {
        let httpclient = reqwest::Client::new();
        let app_id = env!("APP_ID");
        let app_secret = env!("APP_SECRET");

        let auth_client = AuthClient::new(httpclient);
        let tae = auth_client.fetch_access_token(app_id.into(), app_secret.into()).await?;

        assert_eq!(tae.tenant_access_token.len(), 42);
        assert!(tae.expire > 0);

        Ok(())
    }
}
