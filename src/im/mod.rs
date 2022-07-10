pub mod messages;
mod request;
mod response;

use reqwest::header::CONTENT_TYPE;

use self::messages::{Content, Receiver, SendMessageData};
use self::request::SendMessageRequest;
use self::response::SendMessageResponse;
use crate::auth::AuthClient;
use crate::error::Error;
use crate::result::Result;

pub struct IMClient {
    app_id: String,
    app_secret: String,

    auth: AuthClient,
    httpclient: reqwest::Client,
}

impl IMClient {
    pub fn new(app_id: String, app_secret: String, auth: AuthClient, httpclient: reqwest::Client) -> Self {
        Self { app_id, app_secret, auth, httpclient }
    }
}

impl IMClient {
    pub async fn send_message(&self, receiver: Receiver, content: Content) -> Result<SendMessageData> {
        let (receive_id, receive_id_type) = receiver.unpack();

        let tae = self.auth.fetch_access_token(self.app_id.clone(), self.app_secret.clone()).await?;

        let url =
            format!("https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type={}", receive_id_type.as_str());
        let request = SendMessageRequest::new(receive_id, content.to_string(), content.message_type());
        let response = self
            .httpclient
            .post(url)
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .bearer_auth(tae.tenant_access_token)
            .json(&request)
            .send()
            .await?
            .json::<SendMessageResponse>()
            .await?;

        if let Some(data) = response.data {
            return Ok(data);
        }

        Err(Error::LarkBackend(response.code, response.message))
    }
}
