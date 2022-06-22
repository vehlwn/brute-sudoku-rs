FROM rust:1.53.0-alpine

WORKDIR /root/backend-rs
COPY Cargo.toml ./
COPY stub-main.rs ./src/main.rs
RUN apk add musl-dev
RUN cargo build --release

RUN rm ./src/main.rs
RUN rm ./target/release/deps/brute_sudoku_rs*
COPY src ./src
RUN cargo build --release
ENTRYPOINT cargo run --release
