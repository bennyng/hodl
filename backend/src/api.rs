#![warn(clippy::all, clippy::cargo)]

use rocket::response::stream::{Event, EventStream, TextStream};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{http::Status, response::status};

use rocket::tokio::time::{self, Duration};

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    code: String,
    name: String,
}

#[get("/")]
pub fn index() -> status::Accepted<String> {
    status::Accepted(Some(String::from("api root")))
}

// https://github.com/SergioBenitez/Rocket/tree/v0.5-rc/examples/serialization
#[get("/sym")]
pub fn sym() -> status::Custom<Json<Symbol>> {
    let symbol: Symbol = Symbol {
        name: String::from("Bitcoin"),
        code: String::from("btc"),
    };

    println!("{:?}", symbol);

    status::Custom(Status::ImATeapot, Json(symbol))
}

#[get("/hb")]
pub fn heartbeat() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = time::interval(Duration::from_secs(3));
        loop {
            yield "hello";
            interval.tick().await;
        }
    }
}

#[get("/btc")]
pub fn btc() -> EventStream![] {
    let symbol = Symbol {
        name: String::from("Bitcoin"),
        code: String::from("btc"),
    };

    EventStream! {
        loop {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                yield Event::json(&symbol);
                interval.tick().await;
            }
        }
    }
}

#[catch(404)]
pub fn not_found() -> &'static str {
    "api AoA"
}
