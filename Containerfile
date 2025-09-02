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
LABEL maintainer="Mickael PATRON"
LABEL org.opencontainers.image.authors="Mickael PATRON"
LABEL org.opencontainers.image.url="https://github.com/mpatron/rust_server"
LABEL org.opencontainers.image.documentation="https://github.com/mpatron/rust_server/README.md"
LABEL org.opencontainers.image.source="https://github.com/mpatron/rust_server"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.revision="1.0.0"
LABEL org.opencontainers.image.vendor="Mickael PATRON"
LABEL org.opencontainers.image.licenses="MIT"
RUN apk add --no-cache libgcc curl
COPY --from=builder /usr/src/app/target/release/rust_server /usr/local/bin/rust_server

# 🔥 Expose le port 8000
EXPOSE 8000
CMD ["rust_server"]
