# island_explore
A game that takes place on an island

## Install / check required tools
Make sure you have basic tools installed:

- [Rust](https://www.rust-lang.org)
- [Bevy Setup](https://bevyengine.org/learn/book/getting-started/setup/)
- [cargo-make](https://sagiegurari.github.io/cargo-make/)

Add WASM Target : `rustup target add wasm32-unknown-unknown`

If build failed : try to install cargo-watch : `cargo install cargo-watch --locked`

## Run

1. Open a new terminal and run: `cargo make serve`
1. Open a second terminal run: `cargo make watch`

## Lint

Run `cargo make verify` in your terminal to format and lint the code.

## Build for release

`cargo make build_release`