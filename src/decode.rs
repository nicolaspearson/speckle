use serde::de::DeserializeOwned;

use crate::errors::{new_error, ErrorKind, Result};
use crate::header::Header;
use crate::serialize::from_jwt_part_claims;

/// The return type of a successful call to [decode](fn.decode.html).
#[derive(Debug)]
pub struct TokenData<T> {
    /// The decoded JWT header
    pub header: Header,
    /// The decoded JWT claims
    pub claims: T,
}

/// Takes the result of a rsplit and ensure we only get 2 parts else errors if we don't.
macro_rules! expect_two {
    ($iter:expr) => {{
        let mut i = $iter;
        match (i.next(), i.next(), i.next()) {
            (Some(first), Some(second), None) => (first, second),
            _ => return Err(new_error(ErrorKind::InvalidToken)),
        }
    }};
}

/// Decode a JWT without any signature verification/validations.
pub fn decode<T: DeserializeOwned>(token: &str) -> Result<TokenData<T>> {
    let (_, message) = expect_two!(token.rsplitn(2, '.'));
    let (claims, header) = expect_two!(message.rsplitn(2, '.'));
    let header = Header::from_encoded(header)?;

    let decoded_claims: T = from_jwt_part_claims(claims)?;

    Ok(TokenData {
        header,
        claims: decoded_claims,
    })
}
