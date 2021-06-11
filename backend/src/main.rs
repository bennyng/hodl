#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
  )]

#[macro_use]
extern crate rocket;

mod api;
mod hello;

use rocket::{response::content, Request};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[catch(404)]
fn not_found(req: &Request) -> content::Html<String> {
    content::Html(format!(
        "<p>Sorry, but '{}' is not a valid path!</p>
            <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead!!</p>",
        req.uri()
    ))
}

// https://rocket.rs/v0.5-rc/guide/overview/#launching
// #[launch]
// fn rocket() -> _ {
#[rocket::main]
async fn main() {
    let result = rocket::build()
        .mount("/hello", routes![hello::index])
        .register("/hello", catchers![hello::not_found])
        .mount("/api", routes![api::index, api::sym])
        .register("/api", catchers![api::not_found])
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .launch()
        .await;

    match result {
        Ok(_) => println!("Running"),
        Err(e) => println!("Error starting server: {}", e),
    };
}
