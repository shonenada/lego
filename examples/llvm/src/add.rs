use wasmer_llvm_backend::LLVMCompiler;
use wasmer_runtime_core::{import::ImportObject, Func};

static ADD_WASM: &'static [u8] = include_bytes!("../../../src/wasm/add.wasm");

pub fn add(a: i32, b: i32) -> i32 {
    let compiler = &LLVMCompiler::new();
    let module = wasmer_runtime_core::compile_with(ADD_WASM, compiler).expect("should compile");
    let instance = module
        .instantiate(&ImportObject::new())
        .expect("should instantiate");
    let add: Func<(i32, i32), i32> = instance.func("add").unwrap();
    let x = add.call(a, b).unwrap();
    return x;
}
