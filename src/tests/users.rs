pub mod helpers {
    use rand::distributions::Alphanumeric;
    use rand::{rngs::StdRng, Rng, SeedableRng};

    pub async fn registerUser() -> crate::models::UserResp {
        let rng = StdRng::seed_from_u64(124356);
        let rand_string: String = rng.sample_iter(&Alphanumeric).take(12).collect();

        let res = warp::test::request()
            .method("POST")
            .path("/users/register")
            .body(format!(
                r#"{{"email":"{}@gmail.com", "password":"nopass"}}"#,
                rand_string
            ))
            .reply(&crate::router()) // Server routes to respond with
            .await;

        let userResp: crate::models::UserResp = serde_json::from_slice(&res.body()).unwrap();
        userResp
    }
}

pub mod login {
    use super::*;
    // Happy path
    #[tokio::test]
    async fn test__users_login__OK() {
        let user = helpers::registerUser().await.user;
        let res = warp::test::request()
            .method("POST")
            .path("/users/login")
            .body(format!(
                r#"{{"email":"{}", "password":"nopass"}}"#,
                user.email
            ))
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res.status(), 200, "Should return 200 OK.");
        assert_eq!(
            res.body(),
            "{\"user\":{\"email\":\"UEnMNYNMk3qJ@gmail.com\",\"id\":2}}"
        );
    }

    // User doesn't exist in DB
    #[tokio::test]
    async fn test__users_login__WrongUser() {
        let res = warp::test::request()
            .method("POST")
            .path("/users/login")
            .body(r#"{"email":"Icanthazexists@gmail.com", "password":"nopass"}"#)
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res.status(), 401, "Should return 401 Unauthorized.");
        assert_eq!(
            res.body(),
            r#"{"code":401,"message":"Unauthorized","status":"error"}"#
        );
    }

    // GET: Method not allowed
    #[tokio::test]
    async fn test__users_login__WrongMethod() {
        let res = warp::test::request()
            .method("GET")
            .path("/users/login")
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res.status(), 405, "Should return 405 Method not Allowed.");
        assert_eq!(
            res.body(),
            r#"{"code":405,"message":"Method not Allowed","status":"error"}"#
        );
    }
}

pub mod register {
    // Happy path
    #[tokio::test]
    async fn test__users_register__OK() {
        let res = warp::test::request()
            .method("POST")
            .path("/users/register")
            .body(r#"{"email":"will.register@gmail.com", "password":"nopass"}"#)
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res.status(), 200, "Should return 200 OK.");
        assert_eq!(
            res.body(),
            r#"{"user":{"email":"will.register@gmail.com","id":1}}"#
        );
    }

    // register twice: 409 already exists
    #[tokio::test]
    async fn test__users_register__Twice() {
        let _res1 = warp::test::request()
            .method("POST")
            .path("/users/register")
            .body(r#"{"email":"will.register.twice@gmail.com", "password":"nopass"}"#)
            .reply(&crate::router()) // Server routes to respond with
            .await;
        let res2 = warp::test::request()
            .method("POST")
            .path("/users/register")
            .body(r#"{"email":"will.register.twice@gmail.com", "password":"nopass"}"#)
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res2.status(), 409, "Should return 409 Conflict.");
        assert_eq!(
            res2.body(),
            r#"{"code":409,"message":"Already exists","status":"error"}"#
        );
    }
}
