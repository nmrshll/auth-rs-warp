// use crate::routes;

// assert!(
//     warp::test::request()
//         .method("GET")
//         .matches(&get)
//         .await
// );

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

// #[tokio::test]
// async fn getRoot() {
//     // let hello = hello_route::hello()
//     //     .and_then(hello_handler::hello);
//     // let filter = cookieAuthn();

//     assert!(!warp::test::request().path("/1/-5").matches(&filter).await);

//     // let res = warp::test::request()
//     //     .method("GET")
//     //     .path("/hello/www.steadylearner.com") // 1. [Client] - Define request(path with datas) until this
//     //     .reply(&hello!()) // 2. [Server] - How will you respond to it? With what?
//     //     .await;

//     // assert_eq!(res.status(), 200, "Should return 200 OK.");
//     // println!("{:#?}", res.body());
// }
