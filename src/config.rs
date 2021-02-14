use std::env;

use crate::constants;

pub fn api_uri() -> String {
    match env::var("API_URL") {
        Ok(s) if !s.is_empty() => s,
        _ => String::from("127.0.0.1:3000"),
    }
}

pub fn redis_uri() -> String {
    match env::var("REDIS_URL") {
        Ok(s) if !s.is_empty() => s,
        _ => String::from("redis://127.0.0.1:6379"),
    }
}

pub fn environment() -> String {
    match env::var("RUST_ENV") {
        Ok(s) if !s.is_empty() => s,
        _ => String::from(constants::ENV_DEVELOPMENT),
    }
}
