use crate::config;
use crate::db::*;

use actix_files as fs;
use actix_web::{get, http::StatusCode, post, web, web::Form, HttpResponse, Responder};
use serde::Deserialize;
use tera::{Context, Tera};

#[get("/")]
async fn root() -> impl Responder {
    let root_path = config::get_from_env("ROOT_PATH", true);
    fs::NamedFile::open(root_path + "/html/index.html")
}

#[get("/blog")]
async fn blog() -> impl Responder {
    let root_path = config::get_from_env("ROOT_PATH", true);

    let posts = get_posts();

    let mut context = Context::new();
    context.insert("posts", &posts);

    // one-off render blog template with context
    let result = Tera::one_off(
        &(std::fs::read_to_string(root_path + "/templates/blog.html")
            .unwrap_or_else(|e| panic!("Error, couldn't load blog template.\n{}", e))
            .as_str()),
        &context,
        true,
    )
    .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));
    HttpResponse::Ok().content_type("text/html").body(result)
}

#[get("/blog/submit")]
async fn blog_submit() -> impl Responder {
    let root_path = config::get_from_env("ROOT_PATH", true);
    HttpResponse::Ok().set_header("SameSite", "secure").body(
        std::fs::read_to_string(root_path + "/html/submit.html")
            .unwrap_or_else(|e| panic!("Error, couldn't load submit html file.\n{}", e)),
    )
}

#[get("/blog/id/{post_id}")]
async fn blog_permalink(web::Path(post_id): web::Path<std::string::String>) -> impl Responder {
    match post_id.parse::<u32>() {
        Err(_) => HttpResponse::new(StatusCode::NOT_FOUND),
        Ok(i) => {
            let root_path = config::get_from_env("ROOT_PATH", true);

            let post = get_post_by_id(i as i32);

            let mut context = Context::new();
            context.insert("posts", &[post]);

            // one-off render blog template with context
            let result = Tera::one_off(
                &(std::fs::read_to_string(root_path + "/templates/blog.html")
                    .unwrap_or_else(|e| panic!("Error, couldn't load blog template.\n{}", e))
                    .as_str()),
                &context,
                true,
            )
            .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));
            HttpResponse::Ok().content_type("text/html").body(result)
        }
    }
}

#[derive(Deserialize)]
struct NewPostForm {
    title: String,
    body: String,
    token: String,
}

#[post("/blog/posts/new")]
async fn blog_new_post(form: Form<NewPostForm>) -> impl Responder {
    let token = config::get_from_env("SUBMIT_TOKEN", true);

    if form.token == token {
        add_post(&form.title.as_str(), &form.body.as_str());
        println!("New blog post created.");
    } else {
        println!("Unauthorized new blog post");
    }

    HttpResponse::MovedPermanently()
        .set_header("LOCATION", "/blog")
        .finish()
}
