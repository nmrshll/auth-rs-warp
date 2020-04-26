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

// #[cfg(test)]
// mod tests {
//     // Note this useful idiom: importing names from outer (for mod tests) scope.
//     use super::*;

//     // Refer to curl commands in main.rs
//     #[tokio::test]
//     async fn cookieAuthn() {
//         // let hello = hello_route::hello()
//         //     .and_then(hello_handler::hello);
//         let filter = cookieAuthn();

//         assert!(!warp::test::request().path("/1/-5").matches(&filter).await);

//         // let res = warp::test::request()
//         //     .method("GET")
//         //     .path("/hello/www.steadylearner.com")
//         //     .reply(&hello!()) // 2. [Server] - How will you respond to it? With what?
//         //     .await;

//         assert_eq!(res.status(), 200, "Should return 200 OK.");
//         println!("{:#?}", res.body());
//     }
// }
