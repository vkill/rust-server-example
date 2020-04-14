# ExampleServer

## Install rustup and rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
rustup install stable
rustup default stable
```

## Run PostgreSQL and migrate

Ref https://hub.docker.com/_/postgres

Ref https://github.com/golang-migrate/migrate

```
docker rm -f postgres_server_example_development

docker run -d --name postgres_server_example_development --restart always -p 25432:5432 -e POSTGRES_DB=server_example_development -e POSTGRES_USER=example -e POSTGRES_PASSWORD=123456 postgres:12-alpine -c 'log_statement=all'

psql -h 127.0.0.1 -p 25432 -U example server_example_development
```

```
docker run --name postgres_server_example_development_migrate --rm -v $(pwd)/postgres_migrations:/migrations --network host migrate/migrate -path=/migrations -database "postgres://example:123456@127.0.0.1:25432/server_example_development?sslmode=disable" up
```

## Dev

```
cp configuration/base.toml configuration/development.toml
```

```
RUST_BACKTRACE=1 RUST_LOG=debug cargo run
```

```
cargo test --all && \
cargo clippy --all -- -D clippy::all && \
cargo fmt --all -- --check
```
