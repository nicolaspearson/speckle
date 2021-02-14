extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use dotenv::dotenv;
use serde::Serialize;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use std::time::SystemTime;
use warp::http::StatusCode;
use warp::{Filter, Reply};

mod algorithms;
mod constants;
use decode::decode;
mod decode;
use errors::{handle_rejection, WebResult};
mod errors;
mod header;
use jwt::{get_jwt_fixtures, JwtClaims};
mod jwt;
use pool::MobcPool;
mod pool;
mod serialize;

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

    let pool = pool::connect().await.expect("can create mobc pool");

    load_fixtures(pool.clone()).await.expect("fixtures loaded");

    let index_route = warp::path!("jwt")
        .and(with_jwt_extractor())
        .and(with_mobc_pool(pool.clone()))
        .and_then(mobc_handler);

    let routes = index_route.recover(handle_rejection);

    let server: SocketAddr = api_uri().parse().expect("can parse socket address");
    warp::serve(routes).run((server.ip(), server.port())).await;
}

async fn load_fixtures(pool: MobcPool) -> WebResult<impl Reply> {
    let epoch_duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let epoch = &*epoch_duration.as_secs().to_string();
    for jwt in get_jwt_fixtures() {
        debug!("inserting: key: {}; value: {}", jwt, epoch);
        pool::set_str(&pool, &jwt, &epoch, 0)
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

/// An API message serializable to JSON.
#[derive(Serialize)]
struct Message {
    message: String,
}

async fn mobc_handler(jwt: String, pool: MobcPool) -> WebResult<impl Reply> {
    let jwt_claims = decode::<JwtClaims>(&jwt)?;
    let uuid = jwt_claims.claims.uuid;
    let jti = jwt_claims.claims.jti;
    debug!("Finding jwt with uuid: {}, and jti: {}", uuid, jti);
    let key = &*format!("*:*:*:{}:{}:*", uuid, jti);
    let found_match = pool::exists(&pool, String::from(key))
        .await
        .map_err(warp::reject::custom)?;
    let json = warp::reply::json(&Message {
        message: format!("{}", found_match),
    });
    Ok(warp::reply::with_status(json, StatusCode::OK))
}
