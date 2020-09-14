use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use anyhow::Result;
use wamser_runtime::{Compiler, Module};

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
    use wamser_clif_backend::CraneliftCompiler as DefaultCompiler;

    #[cfg(all(not(feature = "backend-clif"), not(feature = "backend-llvm")))]
    use wasmer_clif_backend::CraneliftCompiler as DefaultCompiler;

    return DefaultCompiler::new();
}

#[allow(dead_code)]
pub struct WasmInfo {
    name: String,
    source: String,
    path: String,
    module: Option<Module>,
    mtime: SystemTime,
}

impl WasmInfo {
    pub fn new(
        name: String,
        source: String,
        path: String,
        module: Option<Module>,
        mtime: SystemTime,
    ) -> Self {
        Self {
            module,
            mtime,
            name,
            path,
            source,
        }
    }
}

#[derive(Clone)]
pub struct WasmLoader {
    pub wasm_root: String,
    pub wasms: Arc<RwLock<HashMap<String, WasmInfo>>>,
}

impl WasmLoader {
    pub fn from_env() -> Self {
        let wasm_root = env::var("LEGO_WASM_ROOT").unwrap_or("/opt/lego".to_string());
        Self {
            wasm_root,
            wasms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn load_wasm_from_fs(&mut self, name: &String) -> Result<(Vec<u8>, WasmInfo)> {
        let path = {
            if name.ends_with(".wasm") {
                format!("{}/{}", self.wasm_root, name)
            } else {
                format!("{}/{}.wasm", self.wasm_root, name)
            }
        };

        info!("Loading wasm file from fs: {}", path);
        let wasm_file =
            File::open(path.clone()).map_err(|e| anyhow!("failed to load wasm file: {}", e))?;
        let md = wasm_file.metadata()?;
        let mtime = md.modified()?;
        let mut reader = BufReader::new(wasm_file);
        let mut data = Vec::new();
        reader
            .read_to_end(&mut data)
            .map_err(|e| anyhow!("failed to load wasm: {}", e))?;
        let info = WasmInfo::new(name.clone(), "fs".to_string(), path, None, mtime);
        Ok((data, info));
    }

    pub fn unload_wasm(&mut self, name: &String) -> Result<()> {
        let mut cache = self
            .wasms
            .write()
            .map_err(|e| anyhow!("failed to get WRITE lock of wasm: {}", e))?;
        cache.remove(name);
        Ok(())
    }
}
