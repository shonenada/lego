use serde::{Deserialize, Serialize};

pub static ADD_WASM_PATH: &'static str = "../../src/wasm/add.wasm";
pub static BASE64_WASM_PATH: &'static str = "../../src/wasm/base64.wasm";

#[derive(Serialize, Deserialize)]
pub struct AddRequest {
    pub a: i32,
    pub b: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Base64Request {
    pub text: String,
    pub keyword: String,
    pub username: String,
}
