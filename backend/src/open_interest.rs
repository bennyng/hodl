

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: usize,
    verified: bool,
}

// use rocket::http::Status;
// use rocket::response::status;

// #[get("/<symbol>")]
// pub fn get_open_interest(id: i32) -> Result<Json<Vec<Post>>, Status> {
//   sample::repository::show_posts(&connection)
//     .map(|post| Json(post))
//     .map_err(|error| error_status(error))
// }
