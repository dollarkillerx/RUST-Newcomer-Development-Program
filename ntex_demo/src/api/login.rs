use ntex::{
    web::{
        types::{Json, State},
        Responder,
    },
};
use reqwest::Client;
use std::sync::Arc;
use crate::{AppState, errors::CustomError};
use crate::models::user::{AccessToken, GithubUserInfo, Login};

const CLIENT_ID: &str = "Ov23ct6IBNGJ7yHCsNNy";
const CLIENT_KIC: &str = "3b13d5b4968440a86466b0067b948a10b1c81302";


// 验证github登录跳转过来的
pub async fn github_login(code: Json<Login>, state: State<Arc<AppState>>) -> Result<impl Responder, CustomError> {
    let code = code.0.code;
    let client = Client::new();
    let url = format!("https://github.com/login/oauth/access_token?client_id={}&client_secret={}&code={}", CLIENT_ID, CLIENT_KIC, code);
    let resp = client
        .post(&url)
        .header("Accept", "application/json")
        .send()
        .await;

    let access_token = match resp {
        Ok(resp) => match resp.json::<AccessToken>().await {
            Ok(r) => r.access_token,
            Err(_) => {
                return Err(CustomError::InternalServerError("github login error".to_string()));
            }
        }
        Err(_) => {
            return Err(CustomError::InternalServerError("github login error".to_string()));
        }
    };

    let user_info = client
        .get("https://api.github.com/user")
        .bearer_auth(access_token.clone())
        .header("User-Agent", "blog-ntex")
        .header("Accept", "application/json")
        .send()
        .await;

    let user_info = match user_info {
        Ok(resp) => match resp.json::<GithubUserInfo>().await {
            Ok(r) => r,
            Err(_) => {
                return Err(CustomError::InternalServerError("github login error".to_string()));
            }
        }
        Err(_) => {
            return Err(CustomError::InternalServerError("github login error".to_string()));
        }
    };

    // 存储数据 sql 如果存在则更新 不存在则新增加
    let db_pool = &state.db_pool;
    sqlx::query!("INSERT INTO oauth_user (id, name, avatar_url, create_date) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET name = $2, avatar_url = $3",
        user_info.id as i32, user_info.login, user_info.avatar_url, chrono::Utc::now().naive_utc().date()).execute(db_pool).await?;

    Ok(Json([
        ("access_token", access_token),
    ]))
}