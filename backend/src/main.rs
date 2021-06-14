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

use io::Result;
use rocket::response::content;
use rocket::response::stream::ReaderStream;
use rocket::tokio::net::TcpStream;
use rocket::tokio::time::sleep;
use rocket::tokio::time::Duration;
use rocket::Request;
use std::io;
use std::net::SocketAddr;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// https://rocket.rs/v0.5-rc/guide/responses/#async-streams
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

/// https://rocket.rs/v0.5-rc/guide/responses/#async-streams
#[get("/stream")]
async fn stream() -> Result<ReaderStream![TcpStream]> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    let stream = TcpStream::connect(addr).await?;
    Ok(ReaderStream::one(stream))
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
        .mount(
            "/api",
            routes![api::index, api::sym, api::heartbeat, api::btc],
        )
        .register("/api", catchers![api::not_found])
        .register("/", catchers![not_found])
        .launch()
        .await;

    match result {
        Ok(_) => println!("Running"),
        Err(e) => println!("Error starting server: {}", e),
    };
}
