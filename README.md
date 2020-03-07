# wasm-containerization-talk-examples
Examples featured in my "The Legend of WebAssembly, and the Missing Link" talk. Mostly trying to show off how we can use Wasm and Wasi to containerize applications.

* `writing-a-wasi-application` goes over a how to write a WASI application with the syscalls in [AssemblyScript](https://github.com/AssemblyScript/assemblyscript).
* `compiling-an-application-to-wasi` goes over compiling [QuickJs](https://bellard.org/quickjs/) to WASI using [wasienv](https://github.com/wasienv/wasienv).
* `embedding-wasm-runtime-in-an-application` covered embedding [wasmtime](https://wasmtime.dev/) in a [Rust](https://www.rust-lang.org/) application, to run a wasm module written in rust.
