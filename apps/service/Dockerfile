FROM rust:slim as build

RUN apt-get update; \
  apt-get install --no-install-recommends -y libpq-dev libsqlite3-dev; \
  rm -rf /var/lib/apt/lists/*; \
  USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml

RUN cargo build; \
  rm src/*.rs

COPY ./src ./src
# COPY ./migrations ./migrations
COPY ./diesel.toml .

RUN touch .env; 

RUN rm ./target/debug/deps/service; \
  cargo build --release

FROM debian:buster-slim

RUN mkdir app
WORKDIR /app

RUN apt-get update; \
  apt-get install --no-install-recommends -y libpq5 libsqlite3-0; \
  rm -rf /var/lib/apt/lists/*

COPY --from=build /app/target/release/service .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .

EXPOSE 8000

CMD ["/app/service"]
