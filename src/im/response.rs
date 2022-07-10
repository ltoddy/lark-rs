use serde::Deserialize;

use super::messages::SendMessageData;

#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    pub code: i32,
    #[serde(rename = "msg")]
    pub message: String,
    pub data: Option<SendMessageData>,
}
