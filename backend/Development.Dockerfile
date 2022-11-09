FROM rust:1-slim-buster

WORKDIR /app

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

RUN cargo install cargo-watch

COPY . .

CMD cargo watch -x 'run --bin booksquire-api'
