FROM rust:1.68-alpine AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN docker run --name redis -p 6379:6379 -d redis

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/rust_api .
CMD ["./rust_api"]