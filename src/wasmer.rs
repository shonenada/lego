use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use wasmer_runtime::{instantiate, Array, Func, WasmPtr};
use wasmer_wasi::{generate_import_object_from_state, state::WasiState, WasiVersion};
use crate::types::OutgoingRequest;

const WASI_VERSION: WasiVersion = WasiVersion::Snapshot1;
const WASM_ROOT: &'static str = env!("LEGO_WASM_ROOT");

pub fn load_wasm(path: String) -> Vec<u8> {
    let wasm_file = File::open(format!("{}/{}.wasm", WASM_ROOT, path)).expect("Wasm file");
    let mut reader = BufReader::new(wasm_file);
    let mut data = Vec::new();
    reader.read_to_end(&mut data).expect("Failed to load wasm");
    data
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

    let raw_text = req.text;
    let kw = req.keyword;
    let (_, raw) = raw_text.split_at(kw.len() + 1);

    let get_ptr: Func<(), WasmPtr<u8, Array>> =
        instance.func("getMemoryPtr").expect("getMemoryPtr");
    let buffer_ptr = get_ptr.call().unwrap();

    let memory_writer = buffer_ptr
        .deref(wasm_instance_memory, 0, raw.len() as u32)
        .unwrap();
    for (i, b) in raw.bytes().enumerate() {
        memory_writer[i].set(b);
    }

    let b64encode: Func<u32, u32> = instance.func("b64encode").expect("b64encode");
    let new_len = b64encode.call(raw.len() as u32).unwrap();

    let new_buffer_ptr = get_ptr.call().unwrap();
    let encoded = new_buffer_ptr
        .get_utf8_string(wasm_instance_memory, new_len)
        .unwrap();

    return format!("@{}, Base64.encode(\"{}\") = `{}`", req.username, raw, encoded.to_string());
}
