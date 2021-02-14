use serde::{Deserialize, Serialize};

/// The algorithms supported for signing/verifying JWTs
#[derive(Debug, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    HS256,
    HS384,
    HS512,
    ES256,
    ES384,
    RS256,
    RS384,
    RS512,
    PS256,
    PS384,
    PS512,
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::HS256
    }
}
