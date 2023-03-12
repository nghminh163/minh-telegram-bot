use crate::server::server;

mod config;
mod server;
mod routes;
mod helpers;
pub mod models;
pub mod handlers;
pub mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server().await
}
