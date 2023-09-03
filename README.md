# 2023 Minecraft U Special Projects Camp

## Goal

Chess engine in Rust that compiles to WASM

## Live deployment

[View the game](https://minecraftu.github.io/2023-computer-adventures/)

## Compile & run instructions

```
wasm-pack build --target web --out-dir client/pkg
cd client && python3 -m http.server
```

## Linting

`cargo clippy`

Automatically apply suggestions: `cargo clippy --fix`
