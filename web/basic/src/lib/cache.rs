use redis::Client as RedisClient;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use log;

pub struct Cache {
    client: RedisClient,
}

impl Cache {
    pub fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = RedisClient::open(redis_url)?;
        Ok(Self { client })
    }


    pub async fn set<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        expiration: Duration,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let serialized = serde_json::to_string(value).map_err(|e| {
            log::error!("Failed to serialize value: {:?}", e);
            Box::new(redis::RedisError::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Serialization error",
            ))) as Box<dyn std::error::Error>
        })?;

        conn.set_ex(key, serialized, expiration.as_secs().try_into().unwrap())
            .await?;
        log::info!("Cache set key: {}", key);
        Ok(())
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let value: Option<String> = conn.get(key).await?;

        match value {
            Some(cached) => {
                let deserialized = serde_json::from_str(&cached).map_err(|e| {
                    log::error!("Failed to deserialize value: {:?}", e);
                    Box::new(redis::RedisError::from(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Deserialization error",
                    ))) as Box<dyn std::error::Error>
                })?;
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    pub async fn delete(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        conn.del(key).await?;
        log::info!("Cache deleted key: {}", key);
        Ok(())
    }

}