use redis::{ Client, Commands, RedisResult };
use std::{thread, time::Duration};

pub struct Cache {
    client: Client,
}

impl Cache {
    pub fn new(redis_url: &str) -> RedisResult<Self> {
        const MAX_RETRIES: u32 = 2;
        const RETRY_DELAY: Duration = Duration::from_secs(3);
        
        let mut retry_count = 0;
        
        loop {
            match Client::open(redis_url) {
                Ok(client) => {
                    log::info!("Redis connection established successfully");
                    return Ok(Cache { client });
                }
                Err(e) => {
                    retry_count += 1;
                    log::warn!("Redis connection attempt {} failed: {}", retry_count, e);
                    
                    if retry_count >= MAX_RETRIES {
                        log::error!("Max retries reached. Using fallback mechanism");
                        return Ok(Cache { client: Client::open("redis://127.0.0.1:6379")? });
                    }
                    
                    thread::sleep(RETRY_DELAY);
                }
            }
        }
    }

    pub fn set_value<T: ToString>(&self, key: &str, value: T, expiry_secs: u64) -> RedisResult<()> {
        let result: RedisResult<()> = self.client.get_connection()
            .and_then(|mut con| con.set_ex(key, value.to_string(), expiry_secs));

        match result {
            Ok(_) => {
                log::info!("Value set successfully for key: {}", key);
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to set value for key {}: {}", key, e);
                Ok(())
            }
        }
    }
    
    pub fn get_value(&self, key: &str) -> RedisResult<Option<String>> {
        let result = self.client.get_connection()
            .and_then(|mut con| con.get(key));

        match result {
            Ok(value) => {
                log::info!("Value retrieved successfully for key: {}", key);
                Ok(value)
            }
            Err(e) => {
                log::error!("Failed to get value for key {}: {}", key, e);
                Ok(None)
            }
        }
    }
}
