use anyhow::Result;
use wasmer_runtime::{Instance, Module};
use wasmer_wasi::state::WasiState;
use wasmer_wasi::{generate_import_object_from_state, get_wasi_version, WasiVersion};

pub fn init_wasm_with_version(module: &Module, version: WasiVersion) -> Result<Instance> {
    let state = WasiState::new("lego").build()?;
    let import_object = generate_import_object_from_state(state, version);
    module
        .instantiate(&import_object)
        .map_err(|e| anyhow!("{}", e))
}

pub fn init_wasm(module: &Module) -> Result<Instance> {
    let version = get_wasi_version(module, false);
    match version {
        Some(version) => init_wasm_with_version(module, version),
        None => init_wasm_with_version(module, WasiVersion::Snapshot0),
    }
}
