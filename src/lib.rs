pub mod auth;
pub mod error;
pub mod im;
pub mod result;

use self::auth::AuthClient;
use self::im::IMClient;

#[derive(Debug, Clone)]
pub struct AppConfig {
    app_id: String,
    app_secret: String,
}

impl AppConfig {
    pub fn new(app_id: String, app_secret: String) -> Self {
        Self { app_id, app_secret }
    }
}

pub struct LarkClient {
    pub auth: AuthClient,
    pub im: IMClient,
}

impl LarkClient {
    pub fn new(httpclient: reqwest::Client, conf: AppConfig) -> Self {
        let auth = AuthClient::new(Clone::clone(&httpclient));
        let im = IMClient::new(conf.app_id, conf.app_secret, auth.clone(), httpclient);

        Self { auth, im }
    }
}
