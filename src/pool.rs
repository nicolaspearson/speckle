use crate::redis_uri;
use mobc::{Connection, Pool};
use mobc_redis::redis::AsyncCommands;
use mobc_redis::{redis, RedisConnectionManager};
use std::time::Duration;

use crate::errors::{MobcError::*, Result};

pub type MobcPool = Pool<RedisConnectionManager>;
pub type MobcCon = Connection<RedisConnectionManager>;

const CACHE_POOL_MAX_OPEN: u64 = 16;
const CACHE_POOL_MAX_IDLE: u64 = 8;
const CACHE_POOL_TIMEOUT_SECONDS: u64 = 1;
const CACHE_POOL_EXPIRE_SECONDS: u64 = 60;

pub async fn connect() -> Result<MobcPool> {
    let client = redis::Client::open(redis_uri()).map_err(RedisClientError)?;
    let manager = RedisConnectionManager::new(client);
    Ok(Pool::builder()
        .get_timeout(Some(Duration::from_secs(CACHE_POOL_TIMEOUT_SECONDS)))
        .max_open(CACHE_POOL_MAX_OPEN)
        .max_idle(CACHE_POOL_MAX_IDLE)
        .max_lifetime(Some(Duration::from_secs(CACHE_POOL_EXPIRE_SECONDS)))
        .build(manager))
}

async fn get_con(pool: &MobcPool) -> Result<MobcCon> {
    pool.get().await.map_err(|e| {
        error!("error connecting to redis: {}", e);
        RedisPoolError(e).into()
    })
}

pub async fn set_str(pool: &MobcPool, key: &str, value: &str, ttl_seconds: usize) -> Result<()> {
    let mut con = get_con(&pool).await?;
    con.set(key, value).await.map_err(RedisCMDError)?;
    if ttl_seconds > 0 {
        con.expire(key, ttl_seconds).await.map_err(RedisCMDError)?;
    }
    Ok(())
}

pub async fn exists(pool: &MobcPool, pattern: String) -> Result<bool> {
    let mut con = get_con(&pool).await?;
    let mut iter: redis::AsyncIter<String> =
        con.scan_match(pattern).await.map_err(RedisCMDError)?;
    let mut found_match = false;
    while let Some(element) = iter.next_item().await {
        debug!("Found key: {}", element);
        found_match = true;
    }
    Ok(found_match)
}
