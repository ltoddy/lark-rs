use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Receiver {
    OpenId(String),
    UserId(String),
    UnionId(String),
    Email(String),
    ChatId(String),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverType {
    OpenId,
    UserId,
    UnionId,
    Email,
    ChatId,
}

impl ReceiverType {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReceiverType::OpenId => "open_id",
            ReceiverType::UserId => "user_id",
            ReceiverType::UnionId => "union_id",
            ReceiverType::Email => "email",
            ReceiverType::ChatId => "chat_id",
        }
    }
}

impl Receiver {
    pub fn unpack(self) -> (String, ReceiverType) {
        match self {
            Receiver::OpenId(open_id) => (open_id, ReceiverType::OpenId),
            Receiver::UserId(user_id) => (user_id, ReceiverType::UserId),
            Receiver::UnionId(union_id) => (union_id, ReceiverType::UnionId),
            Receiver::Email(email) => (email, ReceiverType::Email),
            Receiver::ChatId(chat_id) => (chat_id, ReceiverType::ChatId),
        }
    }
}

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

#[derive(Debug, Deserialize)]
pub struct SendMessageData {
    pub body: Body,
    pub chat_id: String,

    // unix timestamp
    pub create_time: String,
    pub deleted: bool,
    pub message_id: String,
    #[serde(rename = "msg_type")]
    pub message_type: MessageType,
    pub sender: Sender,
    // unix timestamp
    pub update_time: String,
    pub updated: bool,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct Sender {
    pub id: String,
    pub id_type: String,
    pub sender_type: String,
    pub tenant_key: String,
}
