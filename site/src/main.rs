mod config;
mod db;
mod routes;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde_derive;
extern crate tera;

use actix_files as fs;
use actix_web::{App, HttpServer};
use config::get_from_env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let root_path = get_from_env("ROOT_PATH", true);

        App::new()
            //.wrap(middleware::NormalizePath::default())
            .service(routes::root)
            .service(routes::blog)
            .service(routes::blog_permalink)
            .service(routes::blog_submit)
            .service(routes::blog_new_post)
            .service(fs::Files::new("/static", root_path + "/static"))
    })
    .bind(String::from("localhost:") + &get_from_env("BIND_PORT", true))?
    .run()
    .await
}
