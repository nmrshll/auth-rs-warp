mod login {
    // Happy path
    // #[tokio::test]
    // async fn test__users_login__OK() {
    //     let res = warp::test::request()
    //         .method("POST")
    //         .path("/users/login")
    //         .body(&"{\"email\":\"Icanthazexists@gmail.com\", \"password\":\"nopass\"}")
    //         .reply(&crate::router()) // Server routes to respond with
    //         .await;
    //     assert_eq!(res.status(), 200, "Should return 200 OK.");
    //     assert_eq!(res.body(), "Hello world !");
    // }

    // User doesn't exist in DB
    #[tokio::test]
    async fn test__users_login__WrongUser() {
        let res = warp::test::request()
            .method("POST")
            .path("/users/login")
            .body(&"{\"email\":\"Icanthazexists@gmail.com\", \"password\":\"nopass\"}")
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res.status(), 401, "Should return 401 Unauthorized.");
        assert_eq!(
            res.body(),
            "{\"code\":401,\"message\":\"Unauthorized\",\"status\":\"error\"}"
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
            "{\"code\":405,\"message\":\"Method not Allowed\",\"status\":\"error\"}"
        );
    }
}

mod register {
    // Happy path
    #[tokio::test]
    async fn test__users_register__OK() {
        let res = warp::test::request()
            .method("POST")
            .path("/users/register")
            .body(&"{\"email\":\"will.register@gmail.com\", \"password\":\"nopass\"}")
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res.status(), 200, "Should return 200 OK.");
        assert_eq!(
            res.body(),
            "{\"user\":{\"email\":\"will.register@gmail.com\",\"id\":1}}"
        );
    }

    // register twice: 409 already exists
    #[tokio::test]
    async fn test__users_register__Twice() {
        let _res1 = warp::test::request()
            .method("POST")
            .path("/users/register")
            .body(&"{\"email\":\"will.register.twice@gmail.com\", \"password\":\"nopass\"}")
            .reply(&crate::router()) // Server routes to respond with
            .await;
        let res2 = warp::test::request()
            .method("POST")
            .path("/users/register")
            .body(&"{\"email\":\"will.register.twice@gmail.com\", \"password\":\"nopass\"}")
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res2.status(), 409, "Should return 409 Conflict.");
        assert_eq!(
            res2.body(),
            "{\"code\":409,\"message\":\"Already exists\",\"status\":\"error\"}"
        );
    }
}
