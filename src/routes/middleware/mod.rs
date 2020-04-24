use warp::cookie;
use warp::{Filter, Rejection};
//
use crate::errors::ServiceError;
use crate::utils::crypto::authn::AuthnToken;

pub fn authn() -> impl Filter<Extract = (AuthnToken,), Error = Rejection> + Copy {
    cookieAuthn()
}

// pub fn headerAuthn() -> impl Filter<Extract = (&'static str,), Error = Rejection> {
//     header::<String>("Authentication").map(|_| "Index page")
//     // warp::path::end().map(|| "Index page")
// }

pub fn cookieAuthn() -> impl Filter<Extract = (AuthnToken,), Error = Rejection> + Copy {
    cookie("token").and_then(cookieAuthnStep2)

    // .ok_or_else(|| warp::reject::missing_cookie("token"))
    // cookie("token").map(move |token_str: String| {
    //     dbg!(&token_str);
    //     let token = AuthnToken::from_str(&token_str).map_err(Rejection::from)?;
    //     // Err(Rejection::from(ServiceError::Unauthorized))
    //     token
    //     // match token.verify(){
    //     //     Ok(_) =>
    //     //     Err(_) => Rejection::from(ServiceError)
    //     // }
    //     // match token.verify() {
    //     //     Ok(_) => return Ok(token),
    //     //     Err(_) => return Err(ServiceError::Unauthorized),
    //     // }
    // })
    // warp::path::end().map(|| "Index page")
}
async fn cookieAuthnStep2(token_str: String) -> Result<AuthnToken, Rejection> {
    let token = AuthnToken::from_str(&token_str).map_err(ServiceError::from)?;
    match token.verify() {
        Ok(_) => return Ok(token),
        Err(_) => return Err(ServiceError::Unauthorized.into()),
    }
}
