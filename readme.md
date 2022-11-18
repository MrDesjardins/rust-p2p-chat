# Rust P2P Chat

Simple experimentation of many workspace using Rust by creating a CLI application that can communicate with other instance of the CLI. The projet does not have a central server but instead use each client to propagate the messages. The projet is an experimentation and is not following any recommended way to build a strong P2P (peer to peer) application.


[<img alt="github" src="https://img.shields.io/badge/github-mrdesjardins/rust_p2p_chat-8dagcb?labelColor=555555&logo=github" height="20">](https://github.com/MrDesjardins/rust-p2p-chat)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rust_p2p_chat.svg?color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rust-p2p-chat)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rust_p2p_chat-66c2a5?labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/rust-p2p-chat/latest/rust_p2p_chat)
[![CI Build](https://github.com/MrDesjardins/rust-p2p-chat/actions/workflows/rust.yml/badge.svg)](https://github.com/MrDesjardins/rust-p2p-chat/actions/workflows/rust.yml)
[![codecov](https://codecov.io/gh/MrDesjardins/rust-p2p-chat/branch/main/graph/badge.svg?token=TWHYC1X1KQ)](https://codecov.io/gh/MrDesjardins/rust-p2p-chat)

# As a Consumer of the Library

## Install
```sh
cargo add rust-p2p-chat
```
To perform test coverage you need to install

```sh
cargo install grcov
rustup component add llvm-tools-preview
```

To generate benchmark plots you need to install GnuPlot

```sh
sudo apt update
sudo apt install gnuplot

# To confirm that it is properly installed:
which gnuplot
```

## How to use?

There is two modes: read and write. The read receives all messages and the write send messages to all the read nodes. If you want to read and write you need two start two instances of the CLI
### Write Mode
```sh
rust-p2p-chat -ip 127.0.0.1 -port 8077 -username YourNameHere -mode w
```

### Read Mode
```sh
rust-p2p-chat -ip 127.0.0.1 -port 8077 -username YourNameHere -mode r
```

# As a Developer of the Library

## What to Install?

```sh
rustup toolchain install stable
rustup default stable
```

## How to Run Locally?

```sh
cargo run -- --ip 127.0.0.1 --port 7777 --username patrick -mode w
```

## Tests

```sh
cargo test
```

## Tests Coverage

You must install few components before running coverage:

```sh
cargo install grcov
rustup component add llvm-tools-preview
```

Then, you can run:

```sh
./coverage.sh
```

Further explanation in the [Mozilla grcov website](https://github.com/mozilla/grcov)

## Documentation

```sh
cargo doc --open
```

# Benchmark

```sh
cargo bench
```

# Publishing

```sh
cargo login
cargo publish --dry-run
cargo publish
```
