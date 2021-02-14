use serde::Serialize;
use std::convert::Infallible;
use std::result;
use thiserror::Error;
use warp::http::StatusCode;
use warp::{Rejection, Reply};

pub type WebResult<T> = result::Result<T, Rejection>;
pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("http error: {0}")]
    HttpError(#[from] HttpError),
    #[error("mobc error: {0}")]
    MobcError(#[from] MobcError),
}

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("base64 error: {0}")]
    Base64DecodeError(base64::DecodeError),
    #[error("json error: {0}")]
    JsonError(serde_json::Error),
    #[error("utf8 error: {0}")]
    Utf8Error(::std::string::FromUtf8Error),
    #[error("jwt token not valid")]
    JWTTokenError,
}

#[derive(Serialize, Debug)]
pub struct HttpErrorResponse {
    message: String,
    status: String,
}

#[derive(Error, Debug)]
pub enum MobcError {
    #[error("could not get redis connection from pool: {0}")]
    RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
    #[error("error executing redis command: {0}")]
    RedisCMDError(mobc_redis::redis::RedisError),
    #[error("error creating Redis client: {0}")]
    RedisClientError(mobc_redis::redis::RedisError),
    #[error("redis key not found")]
    RedisKeyNotFoundError,
}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else {
        debug!("unhandled error: {:?}", err);
        (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
    };

    let json = warp::reply::json(&HttpErrorResponse {
        status: code.to_string(),
        message,
    });

    Ok(warp::reply::with_status(json, code))
}

impl warp::reject::Reject for Error {}
