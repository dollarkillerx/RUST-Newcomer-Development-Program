use serde::{Deserialize, Serialize};
use sea_orm::entity::prelude::*;
use crate::entity::account;
use crate::enums::enums::Direction;
use crate::errors::CustomError;
use crate::errors::CustomError::InternalServerError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionResponse {
    pub client_id: String,                // company.account: exness.10086
    pub subscription_client_id: String,                 // 订阅账户
    pub open_positions: Vec<Positions>,        // 开仓订单
    pub close_position: Vec<Positions>,            // 关仓订单
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BroadcastPayload {
    pub client_id: String,                // company.account: exness.10086
    pub account: Account,                 // 账户信息
    pub positions: Vec<Positions>,        // 持仓
    pub history: Vec<History>,            // 历史订单
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionPayload {
    pub subscription_client_id: String,       // company.account: exness.10086 订阅账户
    pub strategy_code: String,                // 订阅策略code

    pub client_id: String,                // company.account: exness.10086
    pub account: Account,                 // 账户信息
    pub positions: Vec<Positions>,        // 持仓
    pub history: Vec<History>,            // 历史订单
}

impl BroadcastPayload {
    pub fn to_account_entity(&self) -> Result<account::Model, CustomError> {
        let account = account::Model {
            id: xid::new().to_string(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
            client_id: self.client_id.clone(),
            account: self.account.account,
            leverage: self.account.leverage as i32,
            server: self.account.server.clone(),
            company: self.account.company.clone(),
            balance: Decimal::from_f64_retain(self.account.balance).ok_or(InternalServerError("balance is not decimal".to_owned()))?,
            profit: Decimal::from_f64_retain(self.account.profit).ok_or(InternalServerError("profit is not decimal".to_owned()))?,
            margin: Decimal::from_f64_retain(self.account.margin).ok_or(InternalServerError("margin is not decimal".to_owned()))?,
        };

        Ok(account)
    }
}

impl SubscriptionPayload {
    pub fn to_account_entity(&self) -> Result<account::Model, CustomError> {
        let account = account::Model {
            id: xid::new().to_string(),
            created_at: None,
            updated_at: None,
            deleted_at: None,
            client_id: self.client_id.clone(),
            account: self.account.account,
            leverage: self.account.leverage as i32,
            server: self.account.server.clone(),
            company: self.account.company.clone(),
            balance: Decimal::from_f64_retain(self.account.balance).ok_or(InternalServerError("balance is not decimal".to_owned()))?,
            profit: Decimal::from_f64_retain(self.account.profit).ok_or(InternalServerError("profit is not decimal".to_owned()))?,
            margin: Decimal::from_f64_retain(self.account.margin).ok_or(InternalServerError("margin is not decimal".to_owned()))?,
        };

        Ok(account)
    }
}

// Account 账户
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub account: i64,                    // 账户
    pub leverage: i64,                   // 杠杆
    pub server: String,                  // 服务器
    pub company: String,                 // company
    pub balance: f64,                    // 余额
    pub profit: f64,                     // 利润
    pub margin: f64,                     // 预付款
}

// Positions 持仓
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Positions {
    pub order_id: i64,                   // 持仓ID
    pub direction: Direction,            // 方向 (Rust 枚举类型)
    pub symbol: String,                  // 币种
    pub magic: i64,                      // 魔术手
    pub open_price: f64,                 // 开仓价格
    pub volume: f64,                     // 数量
    pub market: f64,                     // 市价
    pub swap: f64,                       // 库存费
    pub profit: f64,                     // 利润
    pub common: String,                  // 注释
    pub opening_time: i64,               // 开仓时间市商
    pub closing_time: i64,               // 平仓时间市商

    pub common_internal: Option<String>,         // 系统内部注释
    pub opening_time_system: Option<i64>,        // 开仓时间系统
    pub closing_time_system: Option<i64>,        // 平仓时间系统
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct History {
    pub ticket: i64,                     // 票据
    pub time_setup: i64,                 // 设置时间
    pub r#type: String,                  // 类型 ("type" 是关键字, 所以使用 r#type)
    pub magic: i64,                      // 魔术手
    pub position_id: i64,                // 持仓ID
    pub volume_initial: f64,             // 初始数量
    pub price_current: f64,              // 当前价格
    pub symbol: String,                  // 币种
    pub comment: String,                 // 注释
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RespResult<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}

impl<T> RespResult<T> {
    pub fn new(code: i32, msg: String, data: T) -> Self {
        Self { code, msg, data }
    }
}

