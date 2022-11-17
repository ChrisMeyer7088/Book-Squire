FROM rust:1-slim-buster as base

ENV USER=root

WORKDIR /var/www/app
RUN cargo init
COPY . .

RUN cargo fetch

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder
RUN cargo build --release --offline

FROM rust:1-slim-buster
WORKDIR /var/www/app

COPY --from=builder /var/www/app/target/release/booksquire-api /usr/bin/booksquire-api

EXPOSE 8080

CMD [ "/usr/bin/booksquire-api" ]
