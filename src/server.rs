use actix_web::{App, HttpServer};

use crate::routes::routes;

pub async fn server() -> std::io::Result<()> {
    let app = || App::new().configure(routes);

    HttpServer::new(app).bind("127.0.0.1:5000")?.run().await
}
