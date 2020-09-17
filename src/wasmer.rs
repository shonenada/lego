use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::RwLock;

use lazy_static::lazy_static;
use rocket::data::Data;
use wasmer_runtime::{error::ResolveResult, Array, Compiler, Func, Instance, Module, WasmPtr};

use crate::init::init_wasm;

lazy_static! {
    static ref WASM_ROOT: String = env::var("LEGO_WASM_ROOT").unwrap();
    static ref WASM_CACHE: RwLock<HashMap<String, Module>> = RwLock::new(HashMap::new());
}

fn default_compiler() -> impl Compiler {
    #[cfg(any(all(
        feature = "backend-llvm",
        feature = "backedn-clif",
        not(feature = "docs")
    )))]
    compile_error!(
        "The `wasmer-backend-X` features are mutually exclusive. Please choose just one"
    );

    #[cfg(all(feature = "backend-llvm"))]
    use wasmer_llvm_backend::LLVMCompiler as DefaultCompiler;

    #[cfg(any(feature = "backend-clif"))]
    use wasmer_clif_backend::CraneliftCompiler as DefaultCompiler;

    #[cfg(all(not(feature = "backend-clif"), not(feature = "backend-llvm")))]
    use wasmer_clif_backend::CraneliftCompiler as DefaultCompiler;

    return DefaultCompiler::new();
}
fn load_wasm(name: String) -> Module {
    let maybe_module = {
        let cache = WASM_CACHE.read().unwrap();
        cache.get(&name).cloned()
    };

    if let Some(module) = maybe_module {
        module.clone()
    } else {
        info!("Load WASM file in {}/{}.wasm", *WASM_ROOT, name);
        let wasm_file = File::open(format!("{}/{}.wasm", *WASM_ROOT, name)).expect("Wasm file");
        let mut reader = BufReader::new(wasm_file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data).expect("Failed to load wasm");
        let compiler = &default_compiler();
        let module = wasmer_runtime_core::compile_with(&data, compiler).expect("should compiled");
        WASM_CACHE.write().unwrap().insert(name, module.clone());
        module
    }
}

fn instantiate_wasm(name: String) -> Instance {
    let module = load_wasm(name);
    init_wasm(&module).unwrap()
}

pub fn get_request(name: String) -> String {
    let instance = instantiate_wasm(name);
    let maybe_handler: ResolveResult<Func<u32, u32>> = instance.exports.get("http_get");
    match maybe_handler {
        Ok(handler) => {
            // Lets get the context and memory of our Wasm Instance
            let wasm_instance_context = instance.context();
            let wasm_instance_memory = wasm_instance_context.memory(0);

            let get_ptr: Func<(), WasmPtr<u8, Array>> =
                instance.exports.get("memory_ptr").expect("memory_ptr");

            let response_len = handler.call(0 as u32).unwrap();
            let buf_ptr = get_ptr.call().unwrap();
            let result = buf_ptr
                .get_utf8_string(wasm_instance_memory, response_len)
                .unwrap();

            result.to_string()
        }
        Err(e) => {
            warn!("Failed to get handler {}", e);
            "Not Found".to_string()
        }
    }
}

pub fn post_request(name: String, data: Data) -> String {
    let instance = instantiate_wasm(name);

    let mut buffer: Vec<u8> = Vec::new();
    data.stream_to(&mut buffer).expect("Read stream");

    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    let get_ptr: Func<(), WasmPtr<u8, Array>> =
        instance.exports.get("memory_ptr").expect("memory_ptr");
    let buffer_ptr = get_ptr.call().unwrap();

    let memory_writer = buffer_ptr
        .deref(wasm_instance_memory, 0, buffer.len() as u32)
        .unwrap();
    for (i, b) in buffer.bytes().enumerate() {
        memory_writer[i].set(b.unwrap());
    }

    let exec_fn: Func<u32, u32> = instance.exports.get("http_post").expect("http_post");
    let new_len = exec_fn.call(buffer.len() as u32).unwrap();

    let new_buffer_ptr = get_ptr.call().unwrap();
    let result_text = new_buffer_ptr
        .get_utf8_string(wasm_instance_memory, new_len)
        .unwrap();

    return result_text.to_string();
}
