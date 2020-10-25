use crate::config;
use crate::db::*;

use actix_files as fs;
use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use tera::{Context, Tera};

pub fn authorized(form_token: &str) -> bool {
    let token = config::get_from_env("SUBMIT_TOKEN", true);
    if token == form_token {
        return true;
    }
    false
}

pub fn id_valid(post_id: String) -> (bool, i32) {
    match post_id.parse::<i32>() {
        Err(_) => (false, 0),
        Ok(id) => {
            if id < 1 {
                (false, id)
            } else {
                (true, id)
            }
        }
    }
}

pub fn replace_newlines(x: &String) -> String {
    x.replace("\n", "<br>")
}

pub fn replace_br_tags(x: &String) -> String {
    x.replace("<br>", "\n")
}

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
        false,
    )
    .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));
    HttpResponse::Ok().content_type("text/html").body(result)
}

#[get("/blog/submit")]
async fn blog_submit() -> impl Responder {
    let root_path = config::get_from_env("ROOT_PATH", true);

    let mut context = Context::new();
    context.insert("title", "");
    context.insert("body", "");

    // one-off render blog template with context
    let result = Tera::one_off(
        &(std::fs::read_to_string(root_path + "/templates/post-submit.html")
            .unwrap_or_else(|e| panic!("Error, couldn't load blog template.\n{}", e))
            .as_str()),
        &context,
        false,
    )
    .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));
    HttpResponse::Ok().content_type("text/html").body(result)
}

#[get("/blog/id/{post_id}")]
async fn blog_by_id(web::Path(post_id): web::Path<std::string::String>) -> impl Responder {
    let (valid, id) = id_valid(post_id);
    if valid {
        let root_path = config::get_from_env("ROOT_PATH", true);

        let post = get_post_by_id(id as i32);

        let mut context = Context::new();
        context.insert("posts", &[post]);

        // one-off render blog template with context
        let result = Tera::one_off(
            &(std::fs::read_to_string(root_path + "/templates/blog.html")
                .unwrap_or_else(|e| panic!("Error, couldn't load blog template.\n{}", e))
                .as_str()),
            &context,
            false,
        )
        .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));
        return HttpResponse::Ok().content_type("text/html").body(result);
    } else {
        return HttpResponse::new(StatusCode::NOT_FOUND);
    }
}

#[get("/blog/edit")]
async fn blog_edit() -> impl Responder {
    "edit"
}

#[get("/blog/edit/{post_id}")]
async fn blog_edit_by_id(web::Path(post_id): web::Path<std::string::String>) -> impl Responder {
    let (valid, id) = id_valid(post_id);
    if valid {
        let root_path = config::get_from_env("ROOT_PATH", true);

        let mut post = get_post_by_id(id as i32);

        post.title = replace_br_tags(&post.title);
        post.body = replace_br_tags(&post.body);

        let mut context = Context::new();
        context.insert("title", &post.title);
        context.insert("body", &post.body);
        context.insert("id", &id);

        // one-off render blog template with context
        let result = Tera::one_off(
            &(std::fs::read_to_string(root_path + "/templates/post-edit.html")
                .unwrap_or_else(|e| panic!("Error, couldn't load blog template.\n{}", e))
                .as_str()),
            &context,
            false,
        )
        .unwrap_or_else(|e| panic!("Error, couldn't render submit template.\n{}", e));

        return HttpResponse::Ok().content_type("text/html").body(result);
    } else {
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }
}
