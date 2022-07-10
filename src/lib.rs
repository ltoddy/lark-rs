pub mod auth;
pub mod error;
pub mod message;
pub mod result;

use self::auth::AuthClient;
use self::message::MessageClient;

pub struct LarkClient {
    pub auth: AuthClient,
    pub message: MessageClient,
}

impl LarkClient {
    pub fn new(httpclient: reqwest::Client) -> Self {
        Self {
            message: MessageClient::new(Clone::clone(&httpclient)),
            auth: AuthClient::new(Clone::clone(&httpclient)),
        }
    }
}
