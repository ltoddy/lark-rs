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
