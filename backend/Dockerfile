# https://kerkour.com/blog/rust-small-docker-image/



FROM rust:1.71.1 AS builder

RUN update-ca-certificates

ENV USER=hodl-backend
ENV UID=10001

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

WORKDIR /hodl-backend

COPY . .

RUN cargo build --release




FROM gcr.io/distroless/cc-debian10

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /hodl-backend

COPY --from=builder /hodl-backend/target/release/hodl-backend ./
COPY --from=builder /hodl-backend/Rocket.toml ./

USER hodl-backend:hodl-backend

CMD ["/hodl-backend/hodl-backend"]