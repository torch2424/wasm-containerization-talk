# Compiling an Application to WASI

In this example, we will be compiling [QuickJS](https://bellard.org/quickjs/) to a [Wasi Compitable QuickJS Wasm module](https://github.com/saghul/wasi-lab/tree/05d2c175afeed626187f792c9dd1a8142e11f95a/qjs-wasi).

The `qjs-wasi` directory is from the MIT licensed [saghul/wasi-lab](https://github.com/saghul/wasi-lab/tree/05d2c175afeed626187f792c9dd1a8142e11f95a/qjs-wasi). 

# Build and Run

Mostly just following the build instructions from `qjs-wasi`.

* Install Python with [pyenv](https://github.com/pyenv/pyenv).
* Install [wasienv](https://github.com/wasienv/wasienv).
* Install [Cmake](https://cmake.org/).
* Install [wasmtime](https://wasmtime.dev/).
* Navigate into the `qjs-wasi` directory: `cd qjs-wasi`.
* Run the build script with `./build.sh`, or do the following:
  * Make a build directory, and navigate to it: `mkdir -p build && cd build`
  * Ran `wasimake` on the parent directory: `wasimake cmake ..`
  * Navigate to the parent directory: `cd ..`
  * Make and output to the build directory: `make -C build`
* Navigate back to this folder: `cd ..` 
* Run the quickjs repl with: `wasmtime qjs-wasi/build/qjs.wasm`
* Run the hello world javascript file (and [grant the capabilities for the filesystem](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md#executing-in-wasmtime-runtime)): `wasmtime qjs-wasi/build/qjs.wasm --dir=. hello-world.js`

