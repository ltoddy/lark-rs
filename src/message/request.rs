use serde::Serialize;

use super::content::MessageType;

#[derive(Debug, Serialize)]
pub struct SendMessageRequest {
    receive_id: String,
    content: String,
    msg_type: MessageType,
}

impl SendMessageRequest {
    pub fn new(receive_id: String, content: String, msg_type: MessageType) -> Self {
        Self { receive_id, content, msg_type }
    }
}
