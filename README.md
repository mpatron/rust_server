# Construction de l'image docker avec podman

~~~bash
podman build -t rust-alpine-app .
podman run --publish 8000:8000 --network host --rm --detach --replace --name rust-alpine-app rust-alpine-app
# podman run --publish 8000:8000                --rm --detach --replace --name rust-alpine-app rust-alpine-app
curl http://127.0.0.1:8000/
podman exec -it rust-alpine-app /bin/sh
~~~
