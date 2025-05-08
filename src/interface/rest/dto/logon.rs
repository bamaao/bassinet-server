use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignInPayload {
    pub request_id: String,
    pub pub_key: String,
    pub sig: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpPayload {
    pub request_id: String,
    pub pub_key: String,
    // // 头像，默认头像地址
    // pub avatar: String,
    // 昵称
    pub nick_name: String,
    pub sig: String,
}