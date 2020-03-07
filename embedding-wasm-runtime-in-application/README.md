# Embedding a Wasm Runtime in an Application

This example shows off embedding a wasm runtime to run wasm modules in a host application. This example essentially goes through [this wasmtime example](https://github.com/bytecodealliance/wasmtime/blob/master/docs/embed-rust.md). 

`guest-wasm-module` is a wasm module written in Rust, compiled with [wasm-pack](https://rustwasm.github.io/wasm-pack/). `host-application` is a rust application, that embeds [wasmtime](https://wasmtime.dev/) to run the `guest-wasm-module`. As a quick aside, `guest-wasm-module` does not use WASI, and is a wasm module that can kind of be seen as a library that would be portable across both the server environment, and the browser.

# Build and Run

* Install [Rust](https://www.rust-lang.org/tools/install).
* Install [wasm-pack](https://rustwasm.github.io/wasm-pack/).
* Navigate into the `guest-wasm-module`: `cd guest-wasm-module`.
* Build the the guest wasm module: `wasm-pack build`.
* Navigate from the `guest-wasm-module` to the `host-application`: `cd .. && cd host-application`.
* Run the host application with: `cargo run`.
