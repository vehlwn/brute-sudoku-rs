FROM rust:1.53.0-alpine AS builder
WORKDIR /root/backend-rs
COPY Cargo.toml ./
COPY stub-main.rs ./src/main.rs
RUN apk add musl-dev
RUN cargo build --release
RUN rm ./src/main.rs ./target/release/deps/brute_sudoku_rs*
COPY src ./src
RUN cargo build --release

FROM alpine
WORKDIR /root/backend-rs
COPY --from=builder /root/backend-rs/target/release/brute-sudoku-rs ./
EXPOSE 5000
ENTRYPOINT ./brute-sudoku-rs
