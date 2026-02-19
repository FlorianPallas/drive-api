database_url := "postgresql://postgres:postgres@localhost:5432/drive"

build:
    cargo build

dev:
    bacon run

generate:
    clorinde live {{ database_url }}

migrate:
    DATABASE_URL={{ database_url }} refinery migrate -e DATABASE_URL
