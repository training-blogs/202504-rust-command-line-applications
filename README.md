# 202504-rust-command-line-applications

Training based on the Command Line Applications in Rust book https://github.com/rust-cli/book

## `hello-rust` Getting started

- https://www.rust-lang.org/learn/get-started

## `grrs` Command Line Applications in Rust

- https://github.com/rust-cli/book
- https://github.com/johnthagen/min-sized-rust

## Cross-Compilation

```shell
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-pc-windows-gnu
```

```shell
rm -rf target/
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
