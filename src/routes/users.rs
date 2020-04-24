use diesel::prelude::*;
use serde_json::json;
use warp::http::Response;
use warp::{Rejection, Reply};
//
use crate::errors::ServiceError;
use crate::models::{NewUser, User, UserResp};
use crate::utils::crypto::{authn::AuthnToken, pwhash};
use crate::utils::db_conn;

#[derive(Debug, Deserialize)]
pub struct RegisterUserIn {
    pub email: String,
    pub password: String,
}

pub async fn user_register(ruIn: RegisterUserIn) -> Result<impl Reply, Rejection> {
    let db = db_conn::get().map_err(ServiceError::from)?;

    let newUser = NewUser::from_credentials(&ruIn.email, &ruIn.password)?;
    let user = newUser.insert(&db)?;

    // set signed cookie with userID
    let token = AuthnToken::from_userId(user.id)?;
    Ok(Response::builder()
        .header("Set-Cookie", token.header_val())
        .body(json!(UserResp::from(user)).to_string()))
}

#[derive(Debug, Deserialize)]
pub struct UserAuthIn {
    pub email: String,
    pub password: String,
}

pub async fn user_login(authIn: UserAuthIn) -> Result<impl Reply, Rejection> {
    let db = db_conn::get().map_err(ServiceError::from)?;

    use crate::schema::users::dsl::{email, users};
    let mut foundUsers = users
        .filter(email.eq(&authIn.email))
        .load::<User>(&db)
        .map_err(ServiceError::from)?;

    let user: User = foundUsers.pop().ok_or(ServiceError::Unauthorized)?;
    pwhash::verify(&user.hash_pass, &authIn.password).or(Err(ServiceError::Unauthorized))?;

    // set signed cookie with userID
    let token = AuthnToken::from_userId(user.id)?;
    dbg!(&token);
    Ok(Response::builder()
        .header("Set-Cookie", token.header_val())
        .body(json!(UserResp::from(user)).to_string()))
}

#[derive(Debug, Deserialize)]
pub struct UserCheckIn {
    pub email: String,
}

pub async fn user_check(checkIn: UserCheckIn) -> Result<impl Reply, Rejection> {
    let db = db_conn::get().map_err(ServiceError::from)?;

    use crate::schema::users::dsl::{email, users};
    let mut foundUsers = users
        .filter(email.eq(&checkIn.email))
        .load::<User>(&db)
        .map_err(ServiceError::from)?;

    let user: User = foundUsers.pop().ok_or(ServiceError::NotFound("user"))?;

    Ok(Response::builder().body(json!(UserResp::from(user)).to_string()))
}
