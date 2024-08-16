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

    pub async fn get_accounts(&self) -> Result<Vec<account::Model>, CustomError> {
        let mut conn = self.redis_conn.get_multiplexed_async_connection().await?;
        let mut position_keys = Vec::new();
        let prefix = CacheKey::CacheAccount.get_key("");
        let mut cursor = 0;

        loop {
            let mut cmd = redis::cmd("SCAN");
            cmd.arg(cursor).arg("MATCH").arg(format!("{}*", prefix)).arg("COUNT").arg(10);
            let (cursor_new, keys): (u64, Vec<String>) = cmd.query_async(&mut conn).await?;

            position_keys.extend(keys);
            cursor = cursor_new;
            if cursor == 0 {
                break;
            }
        }

        let mut result = Vec::new();
        for key in position_keys {
            let sr: Option<String> = conn.get(&key).await?;
            if let Some(sr) = sr {
                match serde_json::from_str::<account::Model>(&sr) {
                    Ok(ac) => result.push(ac),
                    Err(_) => continue, // Skip invalid JSON
                }
            }
        }

        Ok(result)
    }
}