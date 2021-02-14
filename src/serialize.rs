use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::errors::Result;

pub fn b64_decode(input: &str) -> Result<Vec<u8>> {
    base64::decode_config(input, base64::URL_SAFE_NO_PAD).map_err(|e| e.into())
}

/// Decodes from base64 and deserializes from JSON to a struct.
pub fn from_jwt_part_claims<B: AsRef<str>, T: DeserializeOwned>(encoded: B) -> Result<T> {
    let s = String::from_utf8(b64_decode(encoded.as_ref())?)?;
    let claims: T = from_str(&s)?;
    Ok(claims)
}
