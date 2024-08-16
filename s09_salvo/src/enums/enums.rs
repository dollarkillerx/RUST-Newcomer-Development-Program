use std::convert::Infallible;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub enum CacheKey {
    CacheAccount,   // cache 账户信息
    CachePositions, // cache 持仓
    CacheHistory,   // cache 历史持仓
}

impl CacheKey {
    pub fn get_key(self, id: &str) -> String {
        format!("{:?}_{}", self, id)
    }
}

impl std::fmt::Display for CacheKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CacheKey::CacheAccount => write!(f, "CacheAccount"),
            CacheKey::CachePositions => write!(f, "CachePositions"),
            CacheKey::CacheHistory => write!(f, "CacheHistory"),
        }
    }
}

// 定义 Direction 枚举
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Direction {
    BUY,
    SELL,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "BUY" => Ok(Direction::BUY),
            "SELL" => Ok(Direction::SELL),
            _ => unreachable!("Unexpected value encountered"), // 不可能的情况，如果你使用 Infallible 作为错误类型
        }
    }
}

impl From<String> for Direction {
    fn from(s: String) -> Self {
        Direction::from_str(&s).unwrap_or(Direction::BUY) // 默认情况下，如果解析失败，将返回 Direction::BUY
    }
}


impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::BUY => write!(f, "BUY"),
            Direction::SELL => write!(f, "SELL"),
        }
    }
}

