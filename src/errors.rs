use std::error::Error as StdError;
use std::fmt;
use std::result;
use warp::Rejection;

/// A crate private constructor for `Error`.
pub(crate) fn new_error(kind: ErrorKind) -> Error {
    Error(Box::new(kind))
}

pub type WebResult<T> = std::result::Result<T, Rejection>;

/// A type alias for `Result<T, self::Error>`.
pub type Result<T> = result::Result<T, Error>;

/// An error that can occur when encoding/decoding JWTs
#[derive(Debug)]
pub struct Error(Box<ErrorKind>);

impl Error {
    /// Return the specific type of this error.
    pub fn kind(&self) -> &ErrorKind {
        &self.0
    }

    /// Unwrap this error into its underlying type.
    pub fn into_kind(self) -> ErrorKind {
        *self.0
    }
}

/// The specific type of an error.
///
/// This enum may grow additional variants, the `#[non_exhaustive]`
/// attribute makes sure clients don't count on exhaustive matching.
/// (Otherwise, adding a new variant could break existing code.)
#[non_exhaustive]
#[derive(Debug)]
pub enum ErrorKind {
    /// When a token doesn't have a valid JWT shape
    InvalidToken,
    /// When the algorithm from string doesn't match the one passed to `from_str`
    InvalidAlgorithmName,

    // 3rd party errors
    /// An error happened when decoding some base64 text
    Base64(base64::DecodeError),
    /// An error happened while serializing/deserializing JSON
    Json(serde_json::Error),
    /// Some of the text was invalid UTF-8
    Utf8(::std::string::FromUtf8Error),

    // Mobc errors
    // Could not get redis connection from pool
    RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
    // Error parsing string from redis result
    RedisTypeError(mobc_redis::redis::RedisError),
    // Error executing redis command
    RedisCMDError(mobc_redis::redis::RedisError),
    // Error creating Redis client
    RedisClientError(mobc_redis::redis::RedisError),
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match *self.0 {
            ErrorKind::InvalidToken => None,
            ErrorKind::InvalidAlgorithmName => None,
            ErrorKind::Base64(ref err) => Some(err),
            ErrorKind::Json(ref err) => Some(err),
            ErrorKind::Utf8(ref err) => Some(err),
            ErrorKind::RedisPoolError(ref err) => Some(err),
            ErrorKind::RedisTypeError(ref err) => Some(err),
            ErrorKind::RedisCMDError(ref err) => Some(err),
            ErrorKind::RedisClientError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self.0 {
            ErrorKind::InvalidToken | ErrorKind::InvalidAlgorithmName => write!(f, "{:?}", self.0),
            ErrorKind::Base64(ref err) => write!(f, "Base64 error: {}", err),
            ErrorKind::Json(ref err) => write!(f, "JSON error: {}", err),
            ErrorKind::Utf8(ref err) => write!(f, "UTF-8 error: {}", err),
            ErrorKind::RedisPoolError(ref err) => {
                write!(f, "Could not get redis connection from pool: {}", err)
            }
            ErrorKind::RedisTypeError(ref err) => {
                write!(f, "Error parsing string from redis result: {}", err)
            }
            ErrorKind::RedisCMDError(ref err) => {
                write!(f, "Error executing redis command: {}", err)
            }
            ErrorKind::RedisClientError(ref err) => {
                write!(f, " Error creating Redis client: {}", err)
            }
        }
    }
}

impl From<base64::DecodeError> for Error {
    fn from(err: base64::DecodeError) -> Error {
        new_error(ErrorKind::Base64(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        new_error(ErrorKind::Json(err))
    }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(err: ::std::string::FromUtf8Error) -> Error {
        new_error(ErrorKind::Utf8(err))
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        new_error(kind)
    }
}

impl warp::reject::Reject for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_rendering() {
        assert_eq!(
            "InvalidAlgorithmName",
            Error::from(ErrorKind::InvalidAlgorithmName).to_string()
        );
    }
}
