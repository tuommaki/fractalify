# fractalify

Simple program that fractalifies images. This is a PoC to test capabilities of
[Krustlet](https://krustlet.dev/).

## Build Instructions

In order to build this, you need to install a WASI-enabled Rust toolchain:
```
$ rustup target add wasm32-wasi
```

To build `fractalify` specify target for build:
```
$ cargo build --release --target wasm32-wasi
```

Now you should have the WebAssembly module created in: `target/wasm32-wasi/release`:
```
$ file target/wasm32-wasi/release/fractalify.wasm 
target/wasm32-wasi/release/fractalify.wasm: WebAssembly (wasm) binary module version 0x1 (MVP)
```

Once you have WebAssembly module created, you can use
[wasm-to-oci](https://github.com/engineerd/wasm-to-oci) to wrap WASM module
into container image and push it to registry:
```
$ wasm-to-oci push target/wasm32-wasi/release/fractalify.wasm <your registry>/fractalify:v0.1.0
```

## Deploying on Krustlet

<TBD>
