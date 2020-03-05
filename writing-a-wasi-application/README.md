# Writing a WASI Application

This is a WASI "Hello World!" written in [AssemblScript](https://github.com/AssemblyScript/assemblyscript).

It uses the [AssemblyScript WASI Bindings](https://github.com/AssemblyScript/assemblyscript/blob/master/std/assembly/bindings/wasi_snapshot_preview1.ts) directly, to call syscalls from the [WASI API](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-api.md). Normally though, you would want to use a more conienient API like [as-wasi](https://github.com/jedisct1/as-wasi).

## Build and Run

Install AssemblyScript with: `npm install`.

Build the sources with: `npm run build`.

Run the built module with a Wasm runtime like [Wasmtime](https://wasmtime.dev/) with: `wasmtime run build/optimized.wasm`.
