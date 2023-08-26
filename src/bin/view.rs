use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let post_id = args()
        .nth(1)
        .expect("get_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");

    let post = posts
        .find(post_id)
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .first(connection)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post with id: {} has a title: {}", post.id, post.title),
        Ok(None) => println!("Unable to find post {}", post_id),
        Err(_) => println!("An error occured while fetching post {}", post_id),
    }
}