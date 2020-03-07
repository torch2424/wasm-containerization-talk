extern crate wasmtime;
use std::error::Error;
use wasmtime::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Global store for the wasmtime instance
    let store = Store::default();

    // Get our guest wasm module
    let module = Module::from_file(&store, "../guest-wasm-module/pkg/guest_wasm_module_bg.wasm")?;

    // Create our imported host function
    let host_import = Func::wrap0(&store, || {
        println!("host_import called!");
    });

    // Create our wasmtime instance, 
    // and import our host function to our module
    let instance = Instance::new(&module, &[host_import.into()])?;

    // Use the wasmtime API to get our exported add function from
    // our guest wasm module
    let guest_add = instance.get_export("add")
        .expect("export named `add` not found")
        .func()
        .expect("export `add` was not a function");
    let guest_add = guest_add.get2::<i32, i32, i32>()?;

    // Call our add function from the guest wasm module
    let result = guest_add(24, 24)?;
    println!("Answer: {:?}", result);
    
    // Use the wasmtime API to get our exported call_host_import function from
    // our guest wasm module
    let guest_call_host_import = instance.get_export("call_host_import")
        .expect("export named `call_host_import` not found")
        .func()
        .expect("export `call_host_import` was not a function");
    let guest_call_host_import = guest_call_host_import.get0::<_>()?;

    // Call our export call_host_import. This call, should then cause
    // host_import to be called, and log to the console
    guest_call_host_import()?;

    Ok(())
}

