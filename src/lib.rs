pub mod auth;
pub mod message;

use self::message::MessageClient;

pub struct LarkClient {
    pub message: MessageClient,
}

impl LarkClient {
    pub fn new(httpclient: reqwest::Client) -> Self {
        Self { message: MessageClient::new(reqwest::Client::clone(&httpclient)) }
    }
}
