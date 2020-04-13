# ExampleServer

## Install rustup and rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
rustup install stable
rustup default stable
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
