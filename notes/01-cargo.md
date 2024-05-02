# Cargo

Package management tool of rust

## Basic Commands

- `cargo new [project-name] [--bin, --lib]`
    - `--bin`: if the project is a executable
    - `--lib`: if is a library

- `cargo build [--release]`
    - compile the code to executable file stored in `target/debug/`
    - `--release` to optimize the deps (?)

- `cargo run [--release]`
    - build and run the executable


## Dependency

Dependencies are managed by `Cargo.toml` and `Cargo.lock`

- `Cargo.toml`
    - document to describe dependencies
- `Cargo.lock`
    - detailed list of requirements based on the toml file


### Cargo.toml

- Ways to add dependencies
  
    ```
    [dependencies]
    rand = "0.3"
    hammer = { version = "0.5.0"}
    color = { git = "https://github.com/bjz/color-rs" }
    geometry = { path = "crates/geometry" }
    ```