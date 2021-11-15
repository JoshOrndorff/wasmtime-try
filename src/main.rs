use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    let file_name = "../moonbeam/target/release/wbuild/moonbeam-runtime/moonbeam_runtime.wasm";
    println!("Importing");
    let now = std::time::Instant::now();
    let module = Module::from_file(&engine, &file_name)?;
    println!("imported in {} ms", now.elapsed().as_millis());
   // A `Store` is what will own instances, functions, globals, etc. All wasm
    // items are stored within a `Store`, and it's what we'll always be using to
    // interact with the wasm world. Custom data can be stored in stores but for
    // now we just use `()`.
    let mut store = Store::new(&engine, ());

    println!("Instantiate");
    let instance = Instance::new(&mut store, &module,  &[])?;

    println!("getting the func");
    // The `Instance` gives us access to various exported functions and items,
    // which we access here to pull out our `answer` exported function and
    // run it.
    let answer = instance.get_func(&mut store, "validate_transaction")
        .expect("`answer` was not an exported function");


    Ok(())
}
