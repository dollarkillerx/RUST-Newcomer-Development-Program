use redis::AsyncCommands;
use crate::enums::enums::CacheKey;
use crate::errors::CustomError;
use crate::storage::storage::Storage;

impl Storage {
    pub async fn set_close_signal(&self, client_id: &str) -> Result<(), CustomError> {
        let cache_key = CacheKey::CacheCloseSignal.get_key(client_id);
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;

        conn.set(&cache_key, "1").await?;
        Ok(())
    }

    pub async fn get_close_signal(&self, client_id: &str) -> Result<bool, CustomError> {
        let cache_key = CacheKey::CacheCloseSignal.get_key(client_id);
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;
        let model: Result<String, redis::RedisError> = conn.get(&cache_key).await;

        match model {
            Ok(_) => {
                conn.del(cache_key).await?;
                Ok(true)
            },
            Err(_) => Ok(false),
        }
    }
}