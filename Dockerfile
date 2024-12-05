FROM rust:1.68-alpine AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN docker run --name redis -p 6379:6379 -d redis

RUN docker run --name my-postgres \
-e POSTGRES_USER=admin \
-e POSTGRES_PASSWORD=password123 \
-e POSTGRES_DB=rust_api \
-p 5432:5432 \
-d postgres:latest

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/rust_api .
CMD ["./rust_api"]