#![warn(clippy::all, clippy::cargo)]

use rocket::response::stream::{Event, EventStream, TextStream};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::time::{self, Duration};
use rocket::{http::Status, response::status};
use tungstenite::connect;
use tungstenite::Message;
use url::Url;

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
    let _symbol = Symbol {
        name: String::from("Bitcoin"),
        code: String::from("btc"),
    };

    EventStream! {
        let (mut socket, response) = connect(
            Url::parse("wss://ftx.com/ws/").unwrap(),
        )
        .expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }

        let sub_json = r#"
        {"op": "subscribe", "channel": "trades", "market": "BTC-PERP"}
        "#;

        socket.write_message(Message::Text(sub_json.into())).unwrap();

        // TODO ping

        loop {
            let msg = socket.read_message().expect("Error reading message");
            // Received: {"marketUpdate":{"market":{"exchangeId":"25","currencyPairId":"9","marketId":"1258"},"tradesUpdate":{"trades":[{"externalId":"241083690","timestamp":"1623741777","timestampNano":"1623741777000000000","priceStr":"40239.653","amountStr":"0.00177186","amountQuoteStr":"71.29903156458","orderSide":"SELLSIDE"}]}}}
            println!("Received: {}", msg);

            yield Event::data(msg.to_string());
        }
    }
}

#[catch(404)]
pub fn not_found() -> &'static str {
    "api AoA"
}
