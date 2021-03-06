use crate::db::*;
use crate::routes::{id_valid, replace_newlines};
use actix_web::{get, http::StatusCode, post, web, web::Form, HttpResponse, Responder};
use serde::Deserialize;
use super::CONFIG_MAP;

#[derive(Deserialize)]
struct NewPostForm {
    title: String,
    body: String,
    token: String,
}

#[derive(Deserialize)]
struct BlogActionForm {
    token: String,
}

#[post("/api/blog/create")]
async fn blog_create_post(form: Form<NewPostForm>) -> impl Responder {
    if *CONFIG_MAP.read().unwrap().get("SUBMIT_TOKEN").unwrap() == form.token {
        create_post(&form.title.as_str(), replace_newlines(&form.body).as_str());
        println!("New blog post created.");
    } else {
        println!("Unauthorized new blog post");
    }

    HttpResponse::MovedPermanently()
        .set_header("LOCATION", "/blog")
        .finish()
}

#[post("/api/blog/posts/edit/{post_id}")]
async fn blog_edit_post(
    web::Path(post_id): web::Path<std::string::String>,
    form: Form<NewPostForm>,
) -> impl Responder {
    let (valid, id) = id_valid(post_id);
    if valid && *CONFIG_MAP.read().unwrap().get("SUBMIT_TOKEN").unwrap() == form.token {
        edit_post_by_id(
            id as i32,
            &form.title.as_str(),
            replace_newlines(&form.body).as_str(),
        );
        println!("Edited post: {}", id);
    } else {
        println!("Unauthorized blog post edit.");
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    return HttpResponse::MovedPermanently()
        .set_header("LOCATION", "/blog")
        .finish();
}

#[post("/api/blog/posts/delete/{post_id}")]
async fn blog_delete_post(
    web::Path(post_id): web::Path<std::string::String>,
    form: Form<BlogActionForm>,
) -> impl Responder {
    let (valid, id) = id_valid(post_id);
    if valid && *CONFIG_MAP.read().unwrap().get("SUBMIT_TOKEN").unwrap() == form.token {
        println!("Deleted post: {}", id);
        delete_post_by_id(id as i32);
    } else {
        println!("Unauthorized blog post deletion.");
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    return HttpResponse::MovedPermanently()
        .set_header("LOCATION", "/blog")
        .finish();
}

#[post("/api/blog/posts/hide/{post_id}")]
async fn blog_hide_post(
    web::Path(post_id): web::Path<std::string::String>,
    form: Form<BlogActionForm>,
) -> impl Responder {
    let (valid, id) = id_valid(post_id);
    if valid && *CONFIG_MAP.read().unwrap().get("SUBMIT_TOKEN").unwrap() == form.token {
        println!("Hid post: {}", id);
        hide_post_by_id(id as i32);
    } else {
        println!("Unauthorized blog post hiding.");
        return HttpResponse::new(StatusCode::UNAUTHORIZED);
    }

    return HttpResponse::MovedPermanently()
        .set_header("LOCATION", "/blog")
        .finish();
}

#[get("/api/blog/posts")]
async fn blog_get_posts_json() -> impl Responder {
    let posts = get_all_posts();
    HttpResponse::Ok().json(posts)
}
