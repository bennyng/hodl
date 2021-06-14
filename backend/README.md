# This project is created by

```sh
cargo new hodl-backend --bin
```

# Install rust

```sh
brew install rustup

# https://doc.rust-lang.org/edition-guide/rust-2018/rustup-for-managing-rust-versions.html
# https://rust-lang.github.io/rustup-components-history/
rustup toolchain install nightly-2021-05-30
rustup default nightly-2021-05-20
```

## Lint

```sh
rustup component add clippy
cargo clippy
```

```rs
// have this at top of file
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::cargo
  )]
```

# Build

```sh
cargo build
```

# Run

```sh
cargo run
```

# VSCode setup

```sh
code --install-extension matklad.rust-analyzer
code --install-extension vadimcn.vscode-lldb
```

https://stackoverflow.com/questions/46885292/how-to-launch-a-rust-application-from-visual-studio-code

# References

https://www.bybt.com/pro

https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104

https://rust-lang.github.io/rustup-components-history/

## Docker

https://kerkour.com/blog/rust-small-docker-image/

https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/

## Crypto Price API

### LunarCrush

https://lunarcrush.com/developers/docs#assets

### Messari

https://messari.io/api

### CryptoWatch (streaming, WebSocket)

https://docs.cryptowat.ch/websocket-api/

F2INJ703C974UAAG3CNP
iq9kWxgeyxaboRzsIEjhVFlsRIjb6S1mCxL/Xn0C

```sh
wscat --connect "wss://stream.cryptowat.ch/connect?apikey=F2INJ703C974UAAG3CNP"

# https://docs.cryptowat.ch/websocket-api/data-subscriptions
> {"subscribe":{"subscriptions":[{"streamSubscription":{"resource":"pairs:9:trades"}}]}}
> {"subscribe":{"subscriptions":[{"streamSubscription":{"resource":"instruments:9:trades"}}]}}

```

> https://github.com/websockets/wscat

### Coin Market Cap

https://pro.coinmarketcap.com/

```sh
❯ curl -H "X-CMC_PRO_API_KEY: 791711f6-efb5-461c-87ac-3d8d810f71e0" -H "Accept: application/json" -d "start=1&limit=100&convert=USD" -G https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest
```
