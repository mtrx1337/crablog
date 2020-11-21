use crate::db;

use actix_web::{get, http::StatusCode, web, HttpResponse, Error, error};
use tera::Context;
use super::CONFIG_MAP;

/// tests if the post id is a valid i32 integer bigger than zero
/// assert(!(id_valid("2147483648").0))
/// assert(!(id_valid("-1").0))
/// assert(id_valid("1").0))
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

/// replaces the \n character with a <br> html tag
/// assert(replace_newlines("test\ntest") == "test<br>test")
pub fn replace_newlines(x: &str) -> String {
    x.replace("\n", "<br>")
}

/// replaces the \n character with a <br> html tag
/// assert(replace_newlines("test<br>test") == "test\ntest")
pub fn replace_br_tags(x: &str) -> String {
    x.replace("<br>", "\n")
}

#[get("/")]
async fn root(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("username", CONFIG_MAP.read().unwrap().get("USERNAME").unwrap());
    context.insert("email", CONFIG_MAP.read().unwrap().get("EMAIL").unwrap());

    let result = tmpl.render("index.html", &context)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error\n{}", e)))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/blog")]
async fn blog(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let posts = db::get_last_five_posts();

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("username", CONFIG_MAP.read().unwrap().get("USERNAME").unwrap());

    let result = tmpl.render("blog.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/blog/all")]
async fn blog_all(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let posts = db::get_all_posts();

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("username", CONFIG_MAP.read().unwrap().get("USERNAME").unwrap());

    let result = tmpl.render("blog-all-posts.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/blog/id/{post_id}")]
async fn blog_by_id(tmpl: web::Data<tera::Tera>, web::Path(post_id): web::Path<std::string::String>) -> Result<HttpResponse, Error> {
    let (valid, id) = id_valid(post_id);
    if valid {
        let post = db::get_post_by_id(id as i32);

        let mut context = Context::new();
        context.insert("post", &post);
        context.insert("username", CONFIG_MAP.read().unwrap().get("USERNAME").unwrap());

        let result = tmpl.render("blog-by-id.html", &context)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

        return Ok(HttpResponse::Ok().content_type("text/html").body(result))
    } else {
        return Ok(HttpResponse::new(StatusCode::NOT_FOUND))
    }
}

#[get("/blog/submit")]
async fn blog_submit(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("title", "");
    context.insert("body", "");

    let result = tmpl.render("submit.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    return Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/blog/edit")]
async fn blog_edit(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut context = Context::new();
    context.insert("posts", &db::get_all_posts());
    context.insert("username", CONFIG_MAP.read().unwrap().get("USERNAME").unwrap());

    let result = tmpl.render("edit.html", &context)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(result))
}

#[get("/blog/edit/{post_id}")]
async fn blog_edit_by_id(tmpl: web::Data<tera::Tera>, web::Path(post_id): web::Path<std::string::String>) -> Result<HttpResponse, Error> {
    let (valid, id) = id_valid(post_id);
    if valid {
        let mut post = db::get_post_by_id(id as i32);

        post.title = replace_br_tags(&post.title);
        post.body = replace_br_tags(&post.body);

        let mut context = Context::new();
        context.insert("title", &post.title);
        context.insert("body", &post.body);
        context.insert("id", &id);

        let result = tmpl.render("edit-form.html", &context)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;

        Ok(HttpResponse::Ok().content_type("text/html").body(result))
    } else {
        Ok(HttpResponse::new(StatusCode::UNAUTHORIZED))
    }
}
