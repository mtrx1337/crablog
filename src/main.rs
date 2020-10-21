mod db;
mod routes;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde_derive;
extern crate tera;

use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::root)
            .service(routes::blog)
            .service(routes::blog_permalink)
            .service(fs::Files::new("/static", "./static/"))
    })
    .bind("localhost:8000")?
    .run()
    .await
}
