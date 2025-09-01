# Construction de l'image docker avec podman

~~~bash
podman build -t rust-alpine-app .
# podman run --publish 8000 --network host --rm --detach --replace --name rust-alpine-app rust-alpine-app
podman run --publish 8888:8000 --rm --detach --replace --name rust-alpine-app rust-alpine-app
curl http://127.0.0.1:8888/
podman exec -it rust-alpine-app /bin/sh
~~~

## Builder

~~~bash
# https://sagiegurari.github.io/cargo-make/
# https://github.com/sagiegurari/cargo-make
mpatron@mylinux:hello-rocket$ cargo install --no-default-features --force cargo-make
mpatron@mylinux:hello-rocket$ cargo make my-flow
~~~

## Make

~~~bash
cargo install --force cargo-make
cargo make
cargo make container-test
~~~

## Test

[https://www.ruststepbystep.com/how-to-test-axum-apis-unit-and-integration-testing-guide/]()

~~~bash
cargo test --test api_tests
~~~
