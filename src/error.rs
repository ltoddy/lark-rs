use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to send request, {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("occur by lark's error, code is {0}, message is {1}")]
    LarkBackend(i32, String),
}
