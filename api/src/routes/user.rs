
use std::{env, sync::{Arc, Mutex}};
use jsonwebtoken::{encode, EncodingKey, Header};
use poem::{
    handler, http::{StatusCode}, web::{Data, Json}, Error
};
use serde::{Deserialize, Serialize};
use store::{store::Store};
use dotenvy::dotenv;
use crate::{request_input::{CreateUserInput}, request_output::{CreateUserOutput, SignInOutput}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

#[handler]
pub fn sign_up(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) 
    -> Result<Json<CreateUserOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let id = locked_s.sign_up(data.username, data.password)
        .map_err(|_| Error::from_status(StatusCode::CONFLICT))?;
    let response = CreateUserOutput {
        id
    };
    Ok(Json(response))
}

#[handler]
pub fn sign_in(Json(data): Json<CreateUserInput>, Data(s): Data<&Arc<Mutex<Store>>>) -> 
    Result<Json<SignInOutput>, Error> {
    let mut locked_s = s.lock().unwrap();
    let user_id = locked_s.sign_in(data.username, data.password);

    match user_id {
        Ok(user_id) => {
            let my_claims = Claims {
                sub: user_id,
                exp: 111111111111111
            };
            dotenv().ok();
            let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| panic!("Please provide a jwt secret"));
            let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_secret.as_ref()))
                .map_err(|_| Error::from_status(StatusCode::UNAUTHORIZED))?;
            let response = SignInOutput {
                jwt: token
            };
            Ok(Json(response))
        }
        Err(_e) => Err(Error::from_status(StatusCode::UNAUTHORIZED))
    }
}