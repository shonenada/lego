#![feature(test)]

extern crate test;

use wasmer_runtime::{instantiate, imports, Array, Func, WasmPtr};
use wasmer_wasi::{generate_import_object_from_state, state::WasiState, WasiVersion};

const WASI_VERSION: WasiVersion = WasiVersion::Snapshot1;
static WASM: &'static [u8] = include_bytes!("../src/wasm/base64.wasm");

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_instance(b: &mut Bencher) {
        b.iter(|| {
            let state = WasiState::new("LegoOutgoing").build().unwrap();
            // let wasm = # load_wasm(path);
            let import_object = generate_import_object_from_state(state, WASI_VERSION);
            instantiate(WASM, &import_object).unwrap()
        });
    }

    #[bench]
    fn bench_get_memory_ptr(b: &mut Bencher) {
        let state = WasiState::new("LegoOutgoing").build().unwrap();
        let import_object = generate_import_object_from_state(state, WASI_VERSION);
        let instance = instantiate(WASM, &import_object).unwrap();

        b.iter(|| {
            let wasm_instance_context = instance.context();
            let _wasm_instance_memory = wasm_instance_context.memory(0);
            let _req_json = "{\"text\":\"hello\"}";
            let get_ptr: Func<(), WasmPtr<u8, Array>> =
                instance.func("getMemoryPtr").expect("getMemoryPtr");
            let _buffer_ptr = get_ptr.call().unwrap();
        });
    }

    #[bench]
    fn bench_save_into_memory(b: &mut Bencher) {
        let state = WasiState::new("LegoOutgoing").build().unwrap();
        let import_object = generate_import_object_from_state(state, WASI_VERSION);
        let instance = instantiate(WASM, &import_object).unwrap();

        let wasm_instance_context = instance.context();
        let wasm_instance_memory = wasm_instance_context.memory(0);
        let req_json = "{\"text\":\"hello\"}";
        let get_ptr: Func<(), WasmPtr<u8, Array>> =
            instance.func("getMemoryPtr").expect("getMemoryPtr");
        let buffer_ptr = get_ptr.call().unwrap();
        b.iter(|| {
            let memory_writer = buffer_ptr
                .deref(wasm_instance_memory, 0, req_json.len() as u32)
                .unwrap();
            for (i, b) in req_json.bytes().enumerate() {
                memory_writer[i].set(b);
            }
        });
    }

    #[bench]
    fn bench_base64encode(b: &mut Bencher) {
        let state = WasiState::new("LegoOutgoing").build().unwrap();
        let import_object = generate_import_object_from_state(state, WASI_VERSION);
        let instance = instantiate(WASM, &import_object).unwrap();

        let wasm_instance_context = instance.context();
        let wasm_instance_memory = wasm_instance_context.memory(0);
        let req_json = "{\"text\":\"hello\"}";
        let get_ptr: Func<(), WasmPtr<u8, Array>> =
            instance.func("getMemoryPtr").expect("getMemoryPtr");
        let buffer_ptr = get_ptr.call().unwrap();
        let memory_writer = buffer_ptr
            .deref(wasm_instance_memory, 0, req_json.len() as u32)
            .unwrap();
        for (i, b) in req_json.bytes().enumerate() {
            memory_writer[i].set(b);
        }
        b.iter(|| {
            let exec_fn: Func<u32, u32> = instance.func("_outgoing").expect("_outgoing");
            let new_len = exec_fn.call(req_json.len() as u32).unwrap();
            let new_buffer_ptr = get_ptr.call().unwrap();
            let result_text = new_buffer_ptr
                .get_utf8_string(wasm_instance_memory, new_len)
                .unwrap();
        });
    }

}
