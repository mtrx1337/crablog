mod db;
mod routes;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde_derive;
extern crate tera;

use actix_files as fs;
use actix_web::{middleware, App, HttpServer};
use std::string::String;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            //.wrap(middleware::NormalizePath::default())
            .service(routes::root)
            .service(routes::blog)
            .service(routes::blog_permalink)
            .service(routes::blog_submit)
            .service(routes::blog_new_post)
            .service(fs::Files::new("/static", "./static/"))
    })
    .bind("localhost:8000")?
    .run()
    .await
}
