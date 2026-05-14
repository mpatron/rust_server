# Axum testouilles

- Opentelemetry [https://oneuptime.com/blog/post/2026-02-06-opentelemetry-tracing-rust-tracing-crate/view](https://oneuptime.com/blog/post/2026-02-06-opentelemetry-tracing-rust-tracing-crate/view)
- Bof [https://www.shuttle.dev/blog/2023/12/06/using-axum-rust](https://www.shuttle.dev/blog/2023/12/06/using-axum-rust)
- Bon tutorial [https://oneuptime.com/blog/post/2026-02-08-how-to-containerize-an-axum-rust-application-with-docker/view](https://oneuptime.com/blog/post/2026-02-08-how-to-containerize-an-axum-rust-application-with-docker/view)
- Bon tutorial [https://github.com/jrollin/oodini](https://github.com/jrollin/oodini)
- Explication des tests unitaires [https://www.ruststepbystep.com/how-to-test-axum-apis-unit-and-integration-testing-guide/](https://www.ruststepbystep.com/how-to-test-axum-apis-unit-and-integration-testing-guide/)
- Complilation par pseudo-make [https://sagiegurari.github.io/cargo-make/](https://sagiegurari.github.io/cargo-make/)
- Complilation par pseudo-make [https://github.com/sagiegurari/cargo-make](https://github.com/sagiegurari/cargo-make)
- Actix [https://github.com/actix/examples](https://github.com/actix/examples)

## Construction de l'image docker avec podman

~~~bash
podman build --build-arg VERSION="$(git describe --tags --abbrev=0)" -t rust-app .
# podman run --publish 8080 --network host --rm --detach --replace --name rust-app rust-app
podman run --env APP_VERSION="$(git describe --tags --abbrev=0)" --publish 8888:8080 --rm --detach --replace --name rust-app rust-app
podman logs rust-app --follow #Voir la version et autre
curl http://127.0.0.1:8888/
curl http://127.0.0.1:8888/health
podman exec --interactive --tty rust-app /bin/sh # Erreur car il n'y rien, pas de shell.
docker export rust-app | tar -tv # Voir qu'il n'y a rien d'autre que le binaire rust_server
~~~

## Builder

~~~bash
cargo install --no-default-features --force cargo-make
cargo make cargo make container-test # Taille de ~82 MB l'image
# Ou
cargo make container-alpine-test # Taille de ~12 MB l'image
~~~

## Make

~~~bash
cargo install --force cargo-make
cargo make
cargo make container-test
cargo make format
~~~

## Test

~~~bash
cargo test --test api_tests
podman run -it --publish 8888:8080 --rm --replace --name rust-app rust-app
~~~

## Clean podman

~~~bash
#Install podman
podman kill $(podman ps -q)
podman rm $(podman ps -a -q)
podman rmi $(podman images -q)
podman rmi --all --force
podman image prune --all --force
podman container prune --force
podman volume prune --force
podman network prune --force
podman system prune --all --volumes --build 
~~~
