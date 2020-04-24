//
pub mod middleware;
mod users;
pub use users::*;
mod protected;
pub use protected::*;

pub async fn getRoot() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Hello world !")
}
