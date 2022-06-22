FROM rust:1.59.0-alpine AS builder
RUN adduser --home /home/app --disabled-password app
WORKDIR /home/app
COPY --chown=app Cargo.toml ./
COPY --chown=app stub-main.rs ./src/main.rs
RUN apk add musl-dev
USER app
RUN cargo build --release
RUN rm ./src/main.rs ./target/release/deps/brute_sudoku_rs*
COPY --chown=app src ./src
RUN cargo build --release

FROM alpine
RUN adduser --home /home/app --disabled-password app
WORKDIR /home/app
USER app
COPY --chown=app --from=builder /home/app/target/release/brute-sudoku-rs ./
EXPOSE 5000
ENTRYPOINT ./brute-sudoku-rs
