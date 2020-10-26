mod models;
mod schema;

use crate::config;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::*;

/// Returns an SqliteConnection if connection successful.
fn establish_connection() -> SqliteConnection {
    let root_path = config::get_from_env("ROOT_PATH", true);
    let db_path = root_path + "/db.sqlite3";
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error, connection to {} failed.", &db_path))
}

/// Returns all posts
pub fn get_all_posts() -> std::vec::Vec<Post> {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    posts
        .filter(published.eq(true))
        .order(id.desc())
        .load::<Post>(&connection)
        .expect("Error, couldn't load posts.")
}

/// Returns the last five posts.
pub fn get_last_five_posts() -> std::vec::Vec<Post> {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    posts
        .filter(published.eq(true))
        .order(id.desc())
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error, couldn't load posts.")
}

/// Returns the post with the given ID.
pub fn get_post_by_id(post_id: i32) -> Post {
    use schema::posts::dsl::*;
    let connection = establish_connection();
    posts
        .find(post_id)
        .get_result(&connection)
        .expect("Error, couldn't find post.")
}

/// Creates a post and publishes it.
pub fn create_post(title: &str, body: &str) {
    use chrono::prelude::*;
    use schema::posts;

    let connection = establish_connection();

    let new_post = NewPost {
        title,
        body,
        published: &true,
        publish_date: &Utc::now().naive_utc(),
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Error, couldn't insert new Post."));
}

/// Updates a post with the new title and body.
pub fn edit_post_by_id(post_id: i32, new_title: &str, new_body: &str) {
    use schema::posts::dsl::*;
    let connection = establish_connection();

    diesel::update(posts)
        .filter(id.eq(post_id))
        .set((title.eq(new_title), body.eq(new_body)))
        .execute(&connection)
        .expect("Error, couldn't update post.");
}

/// Deletes a post by id.
pub fn delete_post_by_id(post_id: i32) {
    use schema::posts::dsl::*;
    let connection = establish_connection();

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&connection)
        .expect("Error, couldn't update post.");
}

/// Sets the published bool of a post to false.
pub fn hide_post_by_id(post_id: i32) {
    use schema::posts::dsl::*;
    let connection = establish_connection();

    diesel::update(posts)
        .filter(id.eq(post_id))
        .set(published.eq(false))
        .execute(&connection)
        .expect("Error, couldn't update post.");
}
