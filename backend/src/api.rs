#![warn(clippy::all, clippy::cargo)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Symbol {
    code: String,
    name: String,
}

#[get("/")]
pub fn index() -> &'static str {
    "api root"
}

#[get("/sym")]
pub fn sym() -> &'static str {
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
    let symbol: Symbol = serde_json::from_str(json).unwrap();

    println!("{:?}", symbol);

    let mut a = 5;
    let b = 0;
    a += b;
    println!("{:?}", &a);

    "Hello, world!"
    // json!(symbol)
}

#[catch(404)]
pub fn not_found() -> &'static str {
    "api AoA"
}
