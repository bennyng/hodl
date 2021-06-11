#![warn(clippy::all, clippy::cargo)]

#[get("/<name>/<age>")]
pub fn index(name: String, age: u8) -> String {
    format!("Hello {}, you are {} years old !", name, age)
}

#[catch(404)]
pub fn not_found() -> &'static str {
    "
  Hello ???
  
  Expected format:
  
  /hello/<name>/<age>
  "
}
