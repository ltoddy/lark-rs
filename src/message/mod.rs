pub mod content;
pub mod receiver;
mod request;

use reqwest::header::CONTENT_TYPE;

use self::content::Content;
use self::receiver::Receiver;
use self::request::SendMessageRequest;

pub struct MessageClient {
    httpclient: reqwest::Client,
}

impl MessageClient {
    pub fn new(httpclient: reqwest::Client) -> Self {
        Self { httpclient }
    }
}

impl MessageClient {
    pub async fn send_message(&self, receiver: Receiver, content: Content) -> Result<(), ()> {
        let (receive_id, receive_id_type) = receiver.unpack();

        let url =
            format!("https://open.feishu.cn/open-apis/im/v1/messages?receive_id_type={}", receive_id_type.as_str());
        let request = SendMessageRequest::new(receive_id, content.to_string(), content.message_type());

        let response = self
            .httpclient
            .post(url)
            .header(CONTENT_TYPE, "application/json; charset=utf-8")
            .bearer_auth("")
            .json(&request)
            .send()
            .await
            .unwrap();

        println!("response is: {:?}", response.text().await.unwrap());

        Ok(())
    }
}
