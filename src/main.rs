#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
extern crate openssl; // for musl. Before diesel macro imports
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
use std::convert::Infallible;
use std::net::SocketAddr;
use warp::{body, get, path, post, Filter, Reply};
//
mod config;
mod errors;
mod models;
mod routes;
use routes::middleware::authn;
use routes::protected;
mod schema;
mod utils;
use utils::db_conn;

mod tests;

pub fn router(
) -> impl Filter<Extract = impl Reply, Error = Infallible> + Clone + Send + Sync + 'static {
    (get().and(path::end()).and_then(routes::getRoot))
        .or(get().and(path("health")).and_then(routes::getRoot))
        .or(post()
            .and(path!("users" / "register").and(body::json().and_then(routes::user_register)))
            .or(path!("users" / "check").and(body::json().and_then(routes::user_check)))
            .or(path!("users" / "login").and(body::json().and_then(routes::user_login)))
            .or(authn()
                .and(path!("protected"))
                .and(body::json())
                .and_then(protected)))
        .recover(errors::handle_rejection)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // auto-loaded: config, logger, db-conn-pool
    pretty_env_logger::init();
    lazy_static::initialize(&db_conn::DB_CONN_POOL);

    let addr: SocketAddr = ([0, 0, 0, 0], config::port()).into();
    info!("Listening on http://{}", addr);
    warp::serve(router()).run(addr).await;
    Ok(())
}
