# Rogue

2D Roguelike written in Rust using Bevy

## Windows Insutrctions

```shell
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

## WASM Instructions

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-server-runner
cargo run --target wasm32-unknown-unknown
```
