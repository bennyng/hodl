# Crypto exercise with Rust and Svelte

## Backend (Rust)

[Read more](./backend/README.md)

## Web (Svelte)

[Read more](./web/README.md)

## Build & Deploy

```sh
# first time
docker login docker.io -u <username> -p <PAT>
kubectl create secret docker-registry dockerhub-secret \
    --docker-server=docker.io \
    --docker-username=bennynwh \
    --docker-password=$(cat ./.dockerhub_pat)

# build & deploy after
make docker-build
```

## Demo

https://bennyng.github.io/hodl/
