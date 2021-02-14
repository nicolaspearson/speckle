use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::errors::{HttpError::*, Result};

pub fn b64_decode(input: &str) -> Result<Vec<u8>> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD).map_err(|e| Base64DecodeError(e).into())
}

pub fn from_utf8(input: Vec<u8>) -> Result<String> {
    match String::from_utf8(input) {
        Ok(s) => Ok(s),
        Err(e) => Err(Utf8Error(e).into()),
    }
}

pub fn json_from_str<T: DeserializeOwned>(utf8: &str) -> Result<T> {
    match from_str(utf8) {
        Ok(s) => Ok(s),
        Err(e) => Err(JsonError(e).into()),
    }
}

/// Decodes from base64 and deserializes from JSON to a struct.
pub fn from_jwt_part_claims<B: AsRef<str>, T: DeserializeOwned>(encoded: B) -> Result<T> {
    let decoded = b64_decode(encoded.as_ref())?;
    let s = from_utf8(decoded)?;
    Ok(json_from_str(&s)?)
}
