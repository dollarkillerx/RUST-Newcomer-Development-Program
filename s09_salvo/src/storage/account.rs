use redis::AsyncCommands;
use crate::entity::account;
use crate::enums::enums::CacheKey;
use crate::errors::CustomError;
use crate::storage::storage::Storage;

impl Storage {
    pub async fn update_account(&self, account_entity: &account::Model) -> Result<(), CustomError> {
        let cache_key = CacheKey::CacheAccount.get_key(&account_entity.client_id);
        let account_json = serde_json::to_string(&account_entity)?;

        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;

        // 存储到 redis，有效期为一年
        const ONE_YEAR_IN_SECONDS: u64 = 365 * 24 * 60 * 60;
        conn.set_ex(&cache_key, &account_json, ONE_YEAR_IN_SECONDS).await?;
        Ok(())
    }

    pub async fn get_account(&self, client_id: &str) -> Result<account::Model, CustomError> {
        let cache_key = CacheKey::CacheAccount.get_key(client_id);
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;
        let model = conn.get(&cache_key).await
            .map_err(CustomError::from)
            .and_then(|json: String| serde_json::from_str(&json).map_err(CustomError::from))?;
        Ok(model)
    }
}