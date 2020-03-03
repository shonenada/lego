#![feature(test)]
extern crate test;

use wasmer_runtime_core::{import::ImportObject, Func};
use wasmer_llvm_backend::LLVMCompiler;

static ADD_WASM: &'static [u8] = include_bytes!("../src/wasm/add.wasm");

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // It's take tooooooo long time to compile..
    // #[bench]
    #[allow(dead_code)]
    fn bench_llvm_ompile(b: &mut Bencher) {
        b.iter(|| {
            let compiler = &LLVMCompiler::new();
            let _module = wasmer_runtime_core::compile_with(ADD_WASM, compiler).expect("should compile");
        });
    }

    #[bench]
    fn bench_llvm_instance(b: &mut Bencher) {
        let compiler = &LLVMCompiler::new();
        let module = wasmer_runtime_core::compile_with(ADD_WASM, compiler).expect("should compile");
        b.iter(|| {
            let _instance = module.instantiate(&ImportObject::new()).expect("should instantiate");
        });
    }

    #[bench]
    fn bench_llvm_get_add(b: &mut Bencher) {
        let compiler = &LLVMCompiler::new();
        let module = wasmer_runtime_core::compile_with(ADD_WASM, compiler).expect("should compile");
        let instance = module.instantiate(&ImportObject::new()).expect("should instantiate");
        b.iter(|| {
            let _add: Func<(i32, i32), i32> = instance.func("add").unwrap();
        });
    }

    #[bench]
    fn bench_llvm_add(b: &mut Bencher) {
        let compiler = &LLVMCompiler::new();
        let module = wasmer_runtime_core::compile_with(ADD_WASM, compiler).expect("should compile");
        let instance = module.instantiate(&ImportObject::new()).expect("should instantiate");
        let add: Func<(i32, i32), i32> = instance.func("add").unwrap();
        b.iter(|| {
            let result = add.call(5, 5).unwrap();
            assert_eq!(result, 10);
        });
    }
}
