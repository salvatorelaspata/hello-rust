# Hello world in Rust

[DOCS](https://doc.rust-lang.org/cargo/index.html)

## Install Rust

One way to install Rust is using [rustup](https://rustup.rs/), a toolchain manager for Rust.

```bash
curl https://sh.rustup.rs -sSf | sh
```

## Create a new project

```bash
cargo new hello_world
```

> Use --lib to create a library crate
> Use --bin to create a binary crate (default)

## Run

```bash
cargo run
```

## Check & Build

```bash
cargo check
cargo build
```

### Fmt & Clippy

Fmt and Clippy are tools to help you write idiomatic Rust.
Fmt formats your code in a standard way.
Clippy is a collection of lints to catch common mistakes and improve your Rust code.

```bash
cargo fmt
cargo clippy
```

## Test

```bash
cargo test
```

## Credits

- [(Youtube) Introduzione a Rust - with Claudio Vesco](https://www.youtube.com/live/Q4wLZs7pfBU?si=C6Ipb1nvI1D1FdRd)
- [(Youtube) Corso Rust 2023 - Introduzione a Rust](https://youtu.be/ishPqDChRew?si=Eglo4ygbnN8QWFBb)
