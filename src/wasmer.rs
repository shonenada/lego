use lazy_static::lazy_static;
use std::env;
use std::fs::File;
use std::sync::RwLock;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use wasmer_runtime::{instantiate, Array, Func, WasmPtr};
use wasmer_wasi::{generate_import_object_from_state, state::WasiState, WasiVersion};

use crate::types::OutgoingRequest;

const WASI_VERSION: WasiVersion = WasiVersion::Snapshot1;
lazy_static! {
    static ref WASM_ROOT: String = env::var("LEGO_WASM_ROOT").unwrap();
    static ref WASM_CACHE: RwLock<HashMap<String, Vec<u8>>> = RwLock::new(HashMap::new());
}

pub fn load_wasm(path: String) -> Vec<u8> {
    let maybe_data = {
        let cache = WASM_CACHE.read().unwrap();
        cache.get(&path).cloned()
    };

    if let Some(data) = maybe_data {
        data.clone()
    } else {
        info!("Load WASM file in {}/{}.wasm", *WASM_ROOT, path);
        let wasm_file = File::open(format!("{}/{}.wasm", *WASM_ROOT, path)).expect("Wasm file");
        let mut reader = BufReader::new(wasm_file);
        let mut data = Vec::new();
        reader.read_to_end(&mut data).expect("Failed to load wasm");
        WASM_CACHE.write().unwrap().insert(path, data.clone());
        data
    }
}

pub fn process_outgoing(path: String, req: OutgoingRequest) -> String {
    let instance = {
        let state = WasiState::new("LegoOutgoing").build().unwrap();
        let wasm = load_wasm(path);
        let import_object = generate_import_object_from_state(state, WASI_VERSION);
        instantiate(wasm.as_slice(), &import_object).unwrap()
    };

    // Lets get the context and memory of our Wasm Instance
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);

    let req_json = serde_json::to_string(&req).unwrap();

    let get_ptr: Func<(), WasmPtr<u8, Array>> =
        instance.func("getMemoryPtr").expect("getMemoryPtr");
    let buffer_ptr = get_ptr.call().unwrap();

    let memory_writer = buffer_ptr
        .deref(wasm_instance_memory, 0, req_json.len() as u32)
        .unwrap();
    for (i, b) in req_json.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    let exec_fn: Func<u32, u32> = instance.func("_outgoing").expect("_outgoing");
    let new_len = exec_fn.call(req_json.len() as u32).unwrap();

    let new_buffer_ptr = get_ptr.call().unwrap();
    let result_text = new_buffer_ptr
        .get_utf8_string(wasm_instance_memory, new_len)
        .unwrap();

    return result_text.to_string();
}
