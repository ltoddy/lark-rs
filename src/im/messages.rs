use std::collections::HashMap;

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
#[serde(tag = "tag")]
pub enum Element {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "a")]
    Hyperlink { href: String, text: String },
    #[serde(rename = "at")]
    Commat { user_id: String, user_name: String },
    #[serde(rename = "img")]
    Image { image_key: String, width: usize, height: usize },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Post {
    title: String,
    content: Vec<Element>,
}

#[derive(Debug, Deserialize, Serialize, Hash, PartialEq, Eq)]
#[serde(rename = "snake_case")]
pub enum Language {
    ZhCn,
    EnUs,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Content {
    Text(String),
    Post(HashMap<Language, Post>),
    #[serde(rename = "image_key")]
    Image(String),
    #[serde(rename = "file_key")]
    File(String),
    #[serde(rename = "file_key")]
    Audio(String),
    Media {
        file_key: String,
        image_key: String,
    },
    #[serde(rename = "file_key")]
    Sticker(String),
    Interactive(String),
    #[serde(rename = "chat_id")]
    ShareChat(String),
    #[serde(rename = "user_id")]
    ShareUser(String),
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
            Content::Post(_) => MessageType::Post,
            Content::Image(_) => MessageType::Image,
            Content::File(_) => MessageType::File,
            Content::Audio(_) => MessageType::Audio,
            Content::Media { .. } => MessageType::Media,
            Content::Sticker(_) => MessageType::Sticker,
            Content::Interactive(_) => MessageType::Interactive,
            Content::ShareChat(_) => MessageType::ShareChat,
            Content::ShareUser(_) => MessageType::ShareUser,
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
