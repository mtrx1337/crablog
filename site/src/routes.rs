use crate::config;
use crate::db;
use crate::html;

use actix_web::{get, http::StatusCode, web, HttpResponse, Responder};
use tera::{Context, Tera};

/// authorizes a request by comparing it to the SUBMIT_TOKEN environment variable
pub fn authorized(form_token: &str) -> bool {
    let token = config::get_from_env("SUBMIT_TOKEN", true);
    if token == form_token {
        return true;
    }
    false
}

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
async fn root() -> impl Responder {
    let mut context = Context::new();

    context.insert("username", &config::get_from_env("USERNAME", true));
    context.insert("email", &config::get_from_env("EMAIL", true));

    let result = Tera::one_off(
        html::INDEX,
        &context,
        false,
    )
    .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));

    HttpResponse::Ok().content_type("text/html").body(result)
}

#[get("/blog")]
async fn blog() -> impl Responder {
    let posts = db::get_last_five_posts();
    let username = config::get_from_env("USERNAME", true);

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("username", &username);
    context.insert("sitetitle", &format!("{}' blog'", &username));
    context.insert("sitedescription", &format!("Last 5 posts of {}' blog'", &username));

    // one-off render blog template with context
    let result = Tera::one_off(
        html::BLOG,
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
        let post = db::get_post_by_id(id as i32);
        let username = config::get_from_env("USERNAME", true);

        let mut context = Context::new();
        context.insert("posts", &[&post]);
        context.insert("username", &username);
        context.insert("sitetitle", &post.title);
        context.insert("sitedescription", &format!("Last 5 posts of {}' blog'", &username));

        // one-off render blog template with context
        let result = Tera::one_off(
            html::BLOG,
            &context,
            false,
        )
        .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));

        return HttpResponse::Ok().content_type("text/html").body(result);
    } else {
        return HttpResponse::new(StatusCode::NOT_FOUND);
    }
}

#[get("/blog/submit")]
async fn blog_submit() -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "");
    context.insert("body", "");

    // one-off render blog template with context
    let result = Tera::one_off(
        html::SUBMIT,
        &context,
        false,
    )
    .unwrap_or_else(|e| panic!("Error, couldn't render blog template.\n{}", e));

    HttpResponse::Ok().content_type("text/html").body(result)
}

#[get("/blog/edit")]
async fn blog_edit() -> impl Responder {
    let mut context = Context::new();
    context.insert("posts", &db::get_all_posts());
    context.insert("username", &config::get_from_env("USERNAME", true));

    // one-off render blog template with context
    let result = Tera::one_off(
        html::EDIT,
        &context,
        false,
    )
    .unwrap_or_else(|e| panic!("Error, couldn't render submit template.\n{}", e));

    return HttpResponse::Ok().content_type("text/html").body(result);
}

#[get("/blog/edit/{post_id}")]
async fn blog_edit_by_id(web::Path(post_id): web::Path<std::string::String>) -> impl Responder {
    let (valid, id) = id_valid(post_id);
    if valid {
        let mut post = db::get_post_by_id(id as i32);

        post.title = replace_br_tags(&post.title);
        post.body = replace_br_tags(&post.body);

        let mut context = Context::new();
        context.insert("title", &post.title);
        context.insert("body", &post.body);
        context.insert("id", &id);

        // one-off render blog template with context
        let result = Tera::one_off(
            html::POST_EDIT_FORM,
            &context,
            false,
        )
        .unwrap_or_else(|e| panic!("Error, couldn't render submit template.\n{}", e));

        return HttpResponse::Ok().content_type("text/html").body(result);
    } else {
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }
}
