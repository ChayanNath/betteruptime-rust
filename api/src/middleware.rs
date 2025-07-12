use jsonwebtoken::{decode, DecodingKey, Validation};
use poem::{
    http::StatusCode, Error, FromRequest, Request,
    RequestBody, Result
};

use crate::routes::user::Claims;

pub struct UserId(pub String);

impl<'a> FromRequest<'a> for UserId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let auth_header = req
            .headers()
            .get("authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| Error::from_string("Missing authorization header", StatusCode::UNAUTHORIZED))?;

        let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret("thisissupersecret".as_ref()),
            &Validation::default(),
        )
        .map_err(|_| Error::from_string("Invalid token", StatusCode::UNAUTHORIZED))?;
        Ok(UserId(decoded.claims.sub))
    }
}