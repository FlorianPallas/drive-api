build:
    cargo build

dev:
    bacon run

deploy:
    - podman build --tag forgejo.fpallas.dev/fpallas/drive/api:latest .
    - podman push forgejo.fpallas.dev/fpallas/drive/api:latest
