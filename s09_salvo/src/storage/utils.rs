use crate::storage::storage::Storage;
use redis::AsyncCommands;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::errors::CustomError;

impl Storage {
    pub async fn gen_time(&self, key: &str) -> Result<i64, CustomError> {
        let cache_key = format!("GEN_{}", key);
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;

        let result: Option<String> = conn.get(&cache_key).await.ok();

        if let Some(result) = result {
            if let Ok(parsed_time) = result.parse::<i64>() {
                // 更新有效期为30天
                const THIRTY_DAYS_IN_SECONDS: u64 = 30 * 24 * 60 * 60;
                conn.expire(&cache_key, THIRTY_DAYS_IN_SECONDS as i64).await?;
                return Ok(parsed_time);
            }
        }

        // 如果获取失败或者解析错误，设置当前时间并返回
        const THIRTY_DAYS_IN_SECONDS: u64 = 30 * 24 * 60 * 60;
        conn.set_ex(&cache_key, now, THIRTY_DAYS_IN_SECONDS).await?;
        Ok(now)
    }
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn test_gen_time() {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        println!("{}",now);
    }
}