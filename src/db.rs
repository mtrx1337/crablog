mod models;
mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use models::*;

fn establish_connection() -> SqliteConnection {
    let db_path = "db.sqlite3";
    SqliteConnection::establish(&db_path)
        .unwrap_or_else(|_| panic!("Error connection to {}", &db_path))
}

pub fn get_posts() -> std::vec::Vec<Post> {
    use crate::db::schema::posts::dsl::*;
    let connection = establish_connection();
    posts
        .filter(published.eq(true))
        .load::<Post>(&connection)
        .expect("Error loading posts")
}

pub fn get_post_by_id(id: i32) -> Post {
    use crate::db::schema::posts::dsl::*;
    let connection = establish_connection();
    posts
        .find(id)
        .get_result(&connection)
        .expect("Couldn't find post")
}

pub fn add_post(title: &str, body: &str) {
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
        .unwrap_or_else(|_| panic!("Error couldn't insert new Post!"));
}
