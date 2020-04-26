mod login {
    #[tokio::test]
    async fn test_getRoot() {
        let res = warp::test::request()
            .method("GET")
            .path("/users/login") // 1. [Client] - Define request(path with datas) until this
            .reply(&crate::router()) // Server routes to respond with
            .await;
        assert_eq!(res.status(), 200, "Should return 200 OK.");
        assert_eq!(res.body(), "Hello world !");
    }
}
