use serde::Serialize;

/// 账户登录信息
#[derive(Debug, Serialize)]
pub struct AccountInfo {
    pub account_id: String,
    pub nick_name: String,
    pub avatar: String,
    pub wallet_address: Option<String>,
    pub package_id: Option<String>,
}