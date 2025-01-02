use redis::Client as RedisClient;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;
use tracing::{error, info};

pub struct Cache {
    client: RedisClient,
}
