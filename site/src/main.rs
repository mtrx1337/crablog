mod api;
mod db;
mod routes;

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate serde_derive;
extern crate tera;

use actix_files as fs;
use actix_web::{App, HttpServer, middleware::Logger};
use env_logger::Env;
use tera::Tera;
use std::{env, sync::RwLock, collections::HashMap};
use once_cell::sync::Lazy;

pub static CONFIG_MAP: Lazy<RwLock<HashMap<String, String>>> = Lazy::new(|| {
    let mut config: HashMap<String, String> = HashMap::new();
    config.insert(String::from("SUBMIT_TOKEN"), env::var("SUBMIT_TOKEN").expect("SUBMIT_TOKEN variable not set."));
    config.insert(String::from("ROOT_PATH"), env::var("ROOT_PATH").expect("ROOT_PATH variable not set."));
    config.insert(String::from("USERNAME"), env::var("USERNAME").expect("USERNAME variable not set."));
    config.insert(String::from("EMAIL"), env::var("EMAIL").expect("EMAIL variable not set."));
    config.insert(String::from("BIND_PORT"), env::var("BIND_PORT").expect("BIND_PORT variable not set."));
    if let Ok(acc) = env::var("GITHUB_ACCOUNT") {
        config.insert(String::from("GITHUB_ACCOUNT"), acc.clone());
    }
    if let Ok(acc) = env::var("TWITTER_ACCOUNT") {
        config.insert(String::from("TWITTER_ACCOUNT"), acc.clone());
    }
    if let Ok(acc) = env::var("MASTODON_ACCOUNT") {
        config.insert(String::from("MASTODON_ACCOUNT"), acc.clone());
    }
    if let Ok(acc) = env::var("DISCORD_ACCOUNT") {
        config.insert(String::from("DISCORD_ACCOUNT"), acc.clone());
    }
    if let Ok(acc) = env::var("REDDIT_ACCOUNT") {
        config.insert(String::from("REDDIT_ACCOUNT"), acc.clone());
    }
    RwLock::new(config)
});

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {

        let mut tera = Tera::new(format!("{}{}", CONFIG_MAP.read().unwrap().get("ROOT_PATH").unwrap(), "/templates/*").as_str()).unwrap();
        tera.autoescape_on(vec![".sql"]);

        env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();


        App::new()
            .data(tera)
            .service(routes::root)
            .service(routes::blog)
            .service(routes::blog_all)
            .service(routes::blog_by_id)
            .service(routes::blog_submit)
            .service(routes::blog_edit)
            .service(routes::blog_edit_by_id)
            .service(api::blog_get_posts_json)
            .service(api::blog_create_post)
            .service(api::blog_edit_post)
            .service(api::blog_hide_post)
            .service(api::blog_delete_post)
            .service(fs::Files::new("/static", "../content/static"))
            .wrap(Logger::new("%a %r %t"))
    })
    .bind(format!("0.0.0.0:{}", CONFIG_MAP.read().unwrap().get("BIND_PORT").unwrap()))?
    .run()
    .await
}
