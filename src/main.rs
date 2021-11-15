use anyhow::Result;
use wasmtime::*;

fn load(file_name: &str) -> Result<()> {
    // Modules can be compiled through either the text or binary format
    let engine = Engine::default();
    println!("Importing {}...", file_name);
    let now = std::time::Instant::now();
    let module = Module::from_file(&engine, &file_name)?;
    println!("imported in {} ms", now.elapsed().as_millis());
  
    Ok(())
}

fn main() -> Result<()> {
    // TODO: handle errors once "Error: expected 46 imports, found 0" is fixed
    load("../moonbeam/target/release/wbuild/moonbeam-runtime/moonbeam_runtime.wasm");
    load("../moonbeam/target/release/wbuild/moonbase-runtime/moonbase_runtime.wasm");

    Ok(())
}
