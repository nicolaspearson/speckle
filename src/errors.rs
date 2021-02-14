use std::result;
use thiserror::Error;
use warp::Rejection;

pub type WebResult<T> = result::Result<T, Rejection>;
pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("jwt error: {0}")]
    JwtError(#[from] JwtError),
    #[error("lib error: {0}")]
    LibError(#[from] LibError),
    #[error("mobc error: {0}")]
    MobcError(#[from] MobcError),
}

#[derive(Error, Debug)]
pub enum LibError {
    #[error("base64 error: {0}")]
    Base64(base64::DecodeError),
    #[error("json error: {0}")]
    Json(serde_json::Error),
    #[error("utf8 error: {0}")]
    Utf8(::std::string::FromUtf8Error),
}

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("the token doesn't have a valid JWT shape")]
    InvalidToken,
    #[error("the algorithm from string doesn't match the one passed to `from_str`")]
    InvalidAlgorithmName,
}

#[derive(Error, Debug)]
pub enum MobcError {
    #[error("could not get redis connection from pool: {0}")]
    RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
    #[error("error parsing string from redis result: {0}")]
    RedisTypeError(mobc_redis::redis::RedisError),
    #[error("error executing redis command: {0}")]
    RedisCMDError(mobc_redis::redis::RedisError),
    #[error("error creating Redis client: {0}")]
    RedisClientError(mobc_redis::redis::RedisError),
}

impl warp::reject::Reject for Error {}
