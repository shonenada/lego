use anyhow::Result;
use wasmer_runtime::{Func, Instance, Module};
use wasmer_wasi::WasiVersion;

use crate::init::{init_wasm, init_wasm_with_version}

fn call_main(instance: Instance) -> Result<u32> {
    let main: Func<u32, u32> = instance
        .exports
        .get("main")
        .map_err(|e| anyhow!("failed to get `main`: {}", e))?;
    main.call(0 as u32)
        .map_err(|e| anyhow!("failed to call `main`: {}", e))
}

pub fn execute_with_version(module: &Module, version: WasiVersion) -> Result<u32> {
    let instance = init_wasm_with_version(module, version)?;
    call_main(instance)
}

pub fn execute_snapshot1(module: &Module) -> Result<u32> {
    execute_with_version(module, WasiVersion::Snapshot1)
}

pub fn execute_unstable(module: &Module) -> Result<u32> {
    execute_with_version(module, WasiVersion::Snapshot0)
}

pub fn execute(module: &Module) -> Result<u32> {
    let instance = init_wasm(module)?;
    call_main(instance)
}
