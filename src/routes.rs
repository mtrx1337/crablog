use crate::db::*;

use actix_files as fs;
use actix_web::{get, web, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn root() -> impl Responder {
    fs::NamedFile::open("html/index.html")
}

#[get("/blog")]
async fn blog() -> impl Responder {
    let posts = get_posts();

    let mut context = Context::new();
    context.insert("posts", &posts);

    // one-off render blog template with context
    let result = Tera::one_off(
        &(std::fs::read_to_string("templates/blog.html")
            .unwrap_or_else(|_| panic!("Couldn't load blog template."))
            .as_str()),
        &context,
        true,
    )
    .unwrap_or_else(|_| panic!("Couldn't render blog template."));
    HttpResponse::Ok().body(result)
}

#[get("/blog/{post_id}")]
async fn blog_permalink(web::Path(post_id): web::Path<u32>) -> impl Responder {
    let post = get_post_by_id(post_id as i32);

    let mut context = Context::new();
    context.insert("posts", &[post]);

    // one-off render blog template with context
    let result = Tera::one_off(
        &(std::fs::read_to_string("templates/blog.html")
            .unwrap_or_else(|_| panic!("Couldn't load blog template."))
            .as_str()),
        &context,
        true,
    )
    .unwrap_or_else(|_| panic!("Couldn't render blog template."));
    HttpResponse::Ok().body(result)
}
