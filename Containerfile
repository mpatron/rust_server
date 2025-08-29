# Stage 1: Build the Rust app
# FROM rust:1.89-alpine3.22 AS builder
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Stage 2: Create minimal runtime image
# FROM alpine:3.22
FROM alpine

RUN apk add --no-cache libgcc curl

COPY --from=builder /usr/src/app/target/release/rust-server /usr/local/bin/rust-server

# 🔥 Expose le port 8000
EXPOSE 8000

CMD ["rust-server"]
