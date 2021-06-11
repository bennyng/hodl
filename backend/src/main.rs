#![warn(
    clippy::all,
    // clippy::restriction,
    // clippy::pedantic,
    clippy::cargo
)]
// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
// #[macro_use]
// extern crate rocket_contrib;

use std::string;

use rocket::response::content;
// use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

mod api;

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: usize,
    verified: bool,
}

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn index() -> &'static str {
    // let json = r#"
    //     {
    //       "name": "George!",
    //       "age": 27,
    //       "verified": false
    //     }
    // "#;
    // let person: Person = serde_json::from_str(json).unwrap();
    let json = r#"
    {
      "code": "btc",
      "name": "Bitcoin"
    }
"#;
    let symbol: api::Symbol = serde_json::from_str(json).unwrap();

    println!("{:?}", symbol);

    let mut a = 5;
    let b = 0;
    a += b;
    println!("{:?}", &a);

    "Hello, world!"
    // json!(symbol)
}

#[catch(404)]
fn not_found(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!(
        "<p>Sorry, but '{}' is not a valid path!</p>
            <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead!!</p>",
        req.uri()
    ))
}

fn main() {
    let e = rocket::build().mount("/", routes![index]);
    // .register(catchers![not_found])
    // .launch();

    println!("Whoops! Rocket didn't launch!");
    // println!("This went wrong: {}", e);
}
