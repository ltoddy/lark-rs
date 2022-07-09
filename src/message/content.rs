use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Content {
    Text(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Text,
    Post,
    Image,
    File,
    Audio,
    Media,
    Sticker,
    Interactive,
    ShareChat,
    ShareUser,
}

impl Content {
    pub fn message_type(&self) -> MessageType {
        match self {
            Content::Text(_) => MessageType::Text,
        }
    }
}

impl ToString for Content {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
