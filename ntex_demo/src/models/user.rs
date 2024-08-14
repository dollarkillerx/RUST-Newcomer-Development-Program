use serde::{Deserialize, Serialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub avatar_url: String,
    pub create_date: Option<chrono::NaiveDate>,
}

/// 前端 Github 授权登录后传上来的 code
#[derive(Debug, Clone, Deserialize)]
pub struct Login {
    pub code: String,
}

/// Github 返回的 access_token
#[derive(Debug, Clone, Deserialize)]
pub struct AccessToken {
    pub access_token: String,
}

/// Github 返回的用户信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GithubUserInfo {
    /// Github 用户 ID
    pub id: u32,
    /// 用户名(不是昵称)
    pub login: String,
    /// 用户头像的地址
    pub avatar_url: String,
}