#![warn(clippy::all, clippy::cargo)]

#[macro_use]
extern crate rocket;

mod api;
mod hello;

use io::Result;
use rocket::http::Method;
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

use rocket_cors::{AllowedHeaders, AllowedOrigins};

// https://rocket.rs/v0.5-rc/guide/overview/#launching
// #[launch]
// fn rocket() -> _ {
#[rocket::main]
async fn main() -> Result<()> {
    let allowed_origins = AllowedOrigins::some(
        &[
            "https://hodl.commonlab-van.com",
            "https://bennyng.github.io/hodl",
            "http://localhost:3000",
            "http://127.0.0.1:3000",
        ],
        &[
            "^https://(.+).hodl.commonlab-van.com$",
            "http://localhost(.+)",
            "http://127.0.0.1(.+)",
        ],
    );

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error cors");

    let rocket_result = rocket::build()
        .mount("/hello", routes![hello::index])
        .register("/hello", catchers![hello::not_found])
        .mount(
            "/api",
            routes![api::index, api::sym, api::heartbeat, api::btc],
        )
        .register("/api", catchers![api::not_found])
        .mount("/", routes![index, stream, delay])
        .register("/", catchers![not_found])
        .attach(cors)
        .launch()
        .await;

    match rocket_result {
        Ok(_) => println!("Running"),
        Err(e) => println!("Error starting server: {}", e),
    };

    Ok(())
}
