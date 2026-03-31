# ---- Builder ----
FROM rust:1.94-alpine AS builder
ARG VERSION
ENV APP_VERSION=${VERSION}
RUN apk add --no-cache musl-dev build-base upx
WORKDIR /src

# (Optionnel mais conseillé) cache deps
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl || true

# Build réel
COPY . .
RUN cargo test --release
RUN cargo build --release --target x86_64-unknown-linux-musl
# Strip pour réduire (UPX optionnel, attention compat)
RUN strip target/x86_64-unknown-linux-musl/release/rust_server

# ---- Runtime ----
FROM scratch

ARG VERSION

LABEL maintainer="Mickael PATRON"
LABEL org.opencontainers.image.authors="Mickael PATRON"
LABEL org.opencontainers.image.url="https://github.com/mpatron/rust_server"
LABEL org.opencontainers.image.documentation="https://github.com/mpatron/rust_server/README.md"
LABEL org.opencontainers.image.source="https://github.com/mpatron/rust_server"
LABEL org.opencontainers.image.version="${VERSION}"
LABEL org.opencontainers.image.revision="${VERSION}"
LABEL org.opencontainers.image.vendor="Mickael PATRON"
LABEL org.opencontainers.image.licenses="MIT"

# Si HTTPS/TLS : copier les CA (depuis alpine)
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

# Copier le binaire
COPY --from=builder /src/target/x86_64-unknown-linux-musl/release/rust_server /rust_server

# Important : scratch n’a pas /tmp ; si ton app en a besoin :
# WORKDIR / (ou prévoir un volume côté K8s)
USER 65532:65532
ENTRYPOINT ["/rust_server"]
