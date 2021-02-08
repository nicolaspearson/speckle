extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use dotenv::dotenv;
use mobc_pool::MobcPool;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use thiserror::Error;
use warp::{Filter, Rejection, Reply};

mod mobc_pool;

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
    debug!("Starting app");

    let mobc_pool = mobc_pool::connect().await.expect("can create mobc pool");
    let mobc_route = warp::path!("mobc")
        .and(with_mobc_pool(mobc_pool.clone()))
        .and_then(mobc_handler);

    let routes = mobc_route;

    let server: SocketAddr = api_uri().parse().expect("Unable to parse socket address");
    warp::serve(routes).run((server.ip(), server.port())).await;
}

async fn mobc_handler(pool: MobcPool) -> WebResult<impl Reply> {
    mobc_pool::set_str(&pool, "mobc_hello", "mobc_world", 60)
        .await
        .map_err(warp::reject::custom)?;
    let value = mobc_pool::get_str(&pool, "mobc_hello")
        .await
        .map_err(warp::reject::custom)?;
    Ok(value)
}

fn with_mobc_pool(
    pool: MobcPool,
) -> impl Filter<Extract = (MobcPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
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
