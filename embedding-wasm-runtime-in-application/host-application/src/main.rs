extern crate wasmtime;
use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    // A `Store` is a sort of "global object" in a sense, but for now it suffices
    // to say that it's generally passed to most constructors.
    let store = Store::default();

    // We start off by creating a `Module` which represents a compiled form
    // of our input wasm module. In this case it'll be JIT-compiled after
    // we parse the text format.
    let module = Module::from_file(&store, "../guest-wasm-module/pkg/guest_wasm_module_bg.wasm")?;

    // First we can create our `log` function, which will simply print out the
    // parameter it receives.
    let host_import = Func::wrap0(&store, || {
        println!("host_import called!");
    });

    // After we have a compiled `Module` we can then instantiate it, creating
    // an `Instance` which we can actually poke at functions on.
    let instance = Instance::new(&module, &[host_import.into()])?;

    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    let guest_add = instance.get_export("add")
        .expect("export named `add` not found")
        .func()
        .expect("export `add` was not a function");

    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `get0` (in this case asserting
    // it takes no arguments and returns one i32) and then call it.
    let guest_add = guest_add.get2::<i32, i32, i32>()?;

    // And finally we can call our function! Note that the error propagation
    // with `?` is done to handle the case where the wasm function traps.
    let result = guest_add(24, 24)?;
    println!("Answer: {:?}", result);
    
    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    let guest_call_host_import = instance.get_export("call_host_import")
        .expect("export named `call_host_import` not found")
        .func()
        .expect("export `call_host_import` was not a function");

    // There's a few ways we can call the `answer` `Func` value. The easiest
    // is to statically assert its signature with `get0` (in this case asserting
    // it takes no arguments and returns one i32) and then call it.
    let guest_call_host_import = guest_call_host_import.get0::<_>()?;
    guest_call_host_import()?;

    Ok(())
}

