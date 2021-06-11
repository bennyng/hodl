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

// #[get("/infinite-hellos")]
// fn hello() -> TextStream![&'static str] {
//     TextStream! {
//         let mut interval = time::interval(Duration::from_secs(1));
//         loop {
//             yield "hello";
//             interval.tick().await;
//         }
//     }
// }
