use warp::{reply, Rejection};
//
// use crate::errors::ServiceError;
use crate::utils::crypto::authn::AuthnToken as Token;
// use crate::utils::db_conn;

#[derive(Debug, Deserialize)]
pub struct ProtectedIn {}

pub async fn protected(autk: Token, _: ProtectedIn) -> Result<reply::Json, Rejection> {
    // optionally: grab a db connection from the shared pool
    // let db = db_conn::get().map_err(ServiceError::from)?;

    // do something related to the user with ID: autk.claims.userID
    dbg!(&autk.claims.userID);

    Ok(reply::json(&autk.claims.userID))
}
