use serde::{Deserialize, Serialize};

use crate::algorithms::Algorithm;
use crate::errors::Result;
use crate::serialize::{b64_decode, from_utf8, json_from_str};

/// A basic JWT header, the alg defaults to HS256 and typ is
/// automatically set to `JWT`. All the other fields are optional.
#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Header {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<String>,
    pub alg: Algorithm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5u: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5t: Option<String>,
}

impl Header {
    /// Returns a JWT header with the algorithm given
    pub fn new(algorithm: Algorithm) -> Self {
        Header {
            typ: Some("JWT".to_string()),
            alg: algorithm,
            cty: None,
            jku: None,
            kid: None,
            x5u: None,
            x5t: None,
        }
    }

    /// Converts an encoded part into the Header struct if possible
    pub fn from_encoded(encoded_part: &str) -> Result<Self> {
        let decoded = b64_decode(encoded_part)?;
        let s = from_utf8(decoded)?;
        Ok(json_from_str(&s)?)
    }
}

impl Default for Header {
    /// Returns a JWT header using the default Algorithm, HS256
    fn default() -> Self {
        Header::new(Algorithm::default())
    }
}
