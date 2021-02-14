extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use dotenv::dotenv;
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::http::StatusCode;
use warp::{Filter, Reply};

mod algorithms;
use config::{api_uri, environment};
mod config;
mod constants;
use decode::decode;
mod decode;
use errors::{handle_rejection, WebResult};
mod errors;
use fixtures::load_fixtures;
mod fixtures;
mod header;
use jwt::JwtPayload;
mod jwt;
use pool::MobcPool;
mod pool;
mod serialize;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    debug!("starting app");

    let pool = pool::connect().await.expect("can create pool");

    if environment() == constants::ENV_DEVELOPMENT {
        load_fixtures(pool.clone()).await.expect("fixtures loaded");
    }

    let index_route = warp::path!("jwt")
        .and(with_jwt_extractor())
        .and(with_pool(pool.clone()))
        .and_then(index_handler);

    let routes = index_route.recover(handle_rejection);

    let server: SocketAddr = api_uri().parse().expect("can parse socket address");
    warp::serve(routes).run((server.ip(), server.port())).await;
}

fn with_jwt_extractor() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::header::<String>(constants::AUTHORIZATION_HEADER)
        .map(|token: String| token.replace(constants::BEARER_PREFIX, ""))
}

fn with_pool(pool: MobcPool) -> impl Filter<Extract = (MobcPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn index_handler(jwt: String, pool: MobcPool) -> WebResult<impl Reply> {
    let jwt_payload = decode::<JwtPayload>(&jwt)?;
    let uuid = jwt_payload.claims.uuid;
    let jti = jwt_payload.claims.jti;
    debug!("Finding jwt with uuid: {}, and jti: {}", uuid, jti);
    let key = &*format!("*:*:*:{}:{}:*", uuid, jti);
    pool::exists(&pool, String::from(key))
        .await
        .map_err(warp::reject::custom)?;
    Ok(warp::reply::with_status(
        warp::reply::reply(),
        StatusCode::OK,
    ))
}
