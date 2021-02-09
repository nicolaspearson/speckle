extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use dotenv::dotenv;
use jsonwebtoken::dangerous_insecure_decode;
use mobc_pool::MobcPool;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::time::SystemTime;
use thiserror::Error;
use warp::{Filter, Rejection, Reply};

mod constants;
mod jwt_utils;
mod mobc_pool;

// TODO: Remove duplicate
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Jwt {
    pub roles: [String; 3],
    pub uuid: String,
    pub jti: String,
    pub platform: String,
}

type WebResult<T> = std::result::Result<T, Rejection>;
type Result<T> = std::result::Result<T, Error>;

fn api_uri() -> String {
    match env::var("API_URL") {
        Ok(s) if !s.is_empty() => s,
        _ => String::from("127.0.0.1:3000"),
    }
}

fn redis_uri() -> String {
    match env::var("REDIS_URL") {
        Ok(s) if !s.is_empty() => s,
        _ => String::from("redis://127.0.0.1:6379"),
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    debug!("starting app");

    let mobc_pool = mobc_pool::connect().await.expect("can create mobc pool");

    load_fixtures(mobc_pool.clone())
        .await
        .expect("fixtures loaded");

    let index_route = warp::path!("jwt")
        .and(with_jwt_extractor())
        .and(with_mobc_pool(mobc_pool.clone()))
        .and_then(mobc_handler);

    let routes = index_route;

    let server: SocketAddr = api_uri().parse().expect("can parse socket address");
    warp::serve(routes).run((server.ip(), server.port())).await;
}

async fn load_fixtures(pool: MobcPool) -> WebResult<impl Reply> {
    let epoch_duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let epoch = &*epoch_duration.as_secs().to_string();
    for jwt in jwt_utils::get_jwt_fixtures() {
        debug!("inserting: key: {}; value: {}", jwt, epoch);
        mobc_pool::set_str(&pool, &jwt, &epoch, 0)
            .await
            .map_err(warp::reject::custom)?;
    }
    Ok("fixtures loaded")
}

fn with_jwt_extractor() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>(constants::AUTHORIZATION_HEADER)
        .map(|token: String| token.replace(constants::BEARER_PREFIX, ""))
}

fn with_mobc_pool(
    pool: MobcPool,
) -> impl Filter<Extract = (MobcPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn mobc_handler(jwt: String, pool: MobcPool) -> WebResult<impl Reply> {
    debug!("JWT: {}", jwt);
    let jwt_claims = dangerous_insecure_decode::<Jwt>(&jwt).expect("");
    debug!("JWT UUID: {}", jwt_claims.claims.uuid);
    debug!("JWT JTI: {}", jwt_claims.claims.uuid);
    // TODO: Find jwt in redis or throw 401, if auth header is missing continue
    // TODO: See: https://github.com/Keats/jsonwebtoken
    // TODO: Do not panic when invalid return 401
    mobc_pool::set_str(&pool, "mobc_hello", "mobc_world", 60)
        .await
        .map_err(warp::reject::custom)?;
    let value = mobc_pool::get_str(&pool, "mobc_hello")
        .await
        .map_err(warp::reject::custom)?;
    Ok(value)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("mobc error: {0}")]
    MobcError(#[from] MobcError),
}

#[derive(Error, Debug)]
pub enum MobcError {
    #[error("could not get redis connection from pool : {0}")]
    RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
    #[error("error parsing string from redis result: {0}")]
    RedisTypeError(mobc_redis::redis::RedisError),
    #[error("error executing redis command: {0}")]
    RedisCMDError(mobc_redis::redis::RedisError),
    #[error("error creating Redis client: {0}")]
    RedisClientError(mobc_redis::redis::RedisError),
}

impl warp::reject::Reject for Error {}
