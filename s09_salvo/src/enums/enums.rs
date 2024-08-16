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

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::BUY => write!(f, "BUY"),
            Direction::SELL => write!(f, "SELL"),
        }
    }
}