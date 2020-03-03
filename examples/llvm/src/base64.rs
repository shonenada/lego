use wasmer_llvm_backend::LLVMCompiler;
use wasmer_runtime::{Array, Func, WasmPtr};
use wasmer_wasi::{generate_import_object_from_state, state::WasiState, WasiVersion};

const WASI_VERSION: WasiVersion = WasiVersion::Snapshot1;
static BASE64_WASM: &'static [u8] = include_bytes!("../../../src/wasm/base64.wasm");

pub fn b64encode(raw: String) -> String {
    let state = WasiState::new("LegoOutgoing").build().unwrap();
    let compiler = &LLVMCompiler::new();
    let module = wasmer_runtime_core::compile_with(BASE64_WASM, compiler).expect("should compile");
    let import_object = generate_import_object_from_state(state, WASI_VERSION);
    let instance = module
        .instantiate(&import_object)
        .expect("should instantiate");
    let wasm_instance_context = instance.context();
    let wasm_instance_memory = wasm_instance_context.memory(0);
    let req_json = format!(
        "{{\"text\":\"! {}\",\"keyword\":\"!\",\"username\":\"user\"}}",
        raw
    );
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
