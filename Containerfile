# 🦀 Étape 1 : Builder intermédiaire pour mettre en cache les dépendances
FROM rust:latest AS deps

WORKDIR /app

# Copie uniquement les fichiers de dépendances pour les mettre en cache
COPY Cargo.toml Cargo.lock ./

# Création d’un faux fichier source pour que `cargo build` télécharge les dépendances
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Compilation des dépendances uniquement
RUN cargo build --release

# 🧱 Étape 2 : Builder final pour compiler le vrai projet
FROM rust:latest AS builder

WORKDIR /app

# Copie les fichiers du projet
COPY . .

# Copie le cache des dépendances depuis l'étape précédente
COPY --from=deps /app/target /app/target
COPY --from=deps /usr/local/cargo /usr/local/cargo

# Compilation du projet réel
RUN cargo build --release

# 🚀 Étape 3 : Image finale minimale
FROM debian:buster-slim
LABEL maintainer="Mickael PATRON"
LABEL org.opencontainers.image.authors="Mickael PATRON"
LABEL org.opencontainers.image.url="https://github.com/mpatron/rust_server"
LABEL org.opencontainers.image.documentation="https://github.com/mpatron/rust_server/README.md"
LABEL org.opencontainers.image.source="https://github.com/mpatron/rust_server"
LABEL org.opencontainers.image.version="1.0.0"
LABEL org.opencontainers.image.revision="1.0.0"
LABEL org.opencontainers.image.vendor="Mickael PATRON"
LABEL org.opencontainers.image.licenses="MIT"

# Création d’un utilisateur non-root (optionnel mais recommandé)
RUN useradd -m appuser

# Copie du binaire compilé
COPY --from=builder /app/target/release/rust_server /usr/local/bin/app

USER appuser

CMD ["app"]
