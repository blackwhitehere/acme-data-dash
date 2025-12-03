# acme-rust-template

Template for a rust application

## Features

- **File-based Logging**: Logs are written to `app.log` (automatically ignored by git)
- Uses `env_logger` and `log` crates for flexible logging
- Log level can be controlled via `RUST_LOG` environment variable (e.g., `RUST_LOG=debug cargo run`)

# Cargo commands

## Check for compile errors:

`cargo check`

## Format files

`cargo fmt`

## Build binaries

`cargo build`

## Run binary

`RUST_LOG=debug cargo run`

## Build documentation

`cargo doc --open`

## Run tests

`cargo test`

## Run benchmarks

Relies on `criterion` library

`cargo bench`

## Profile application

Install `samply`: https://github.com/mstange/samply

`cargo build --profile profiling`
`samply record target/profiling/acme-rust-template`

## Linting
Install `clippy`: `rustup component add clippy`
`cargo clippy --all-targets --all-features -- -D warnings`
