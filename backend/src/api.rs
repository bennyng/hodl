#![warn(clippy::all, clippy::cargo)]

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{http::Status, response::status};

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

#[catch(404)]
pub fn not_found() -> &'static str {
    "api AoA"
}
