#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate serde_derive;

mod llvm_wasmer;
mod types;
mod wasmer;

use base64::encode;
use rocket_contrib::json::{Json, JsonValue};

use crate::llvm_wasmer::{llvm_add, llvm_base64};
use crate::types::{AddRequest, Base64Request};
use crate::wasmer::{add, base64};

#[post("/native/add", format = "json", data = "<message>")]
fn native_add(message: Json<AddRequest>) -> JsonValue {
    let message = message.0;
    json!({
        "result": (message.a + message.b),
    })
}

#[post("/wasm/add", format = "json", data = "<message>")]
fn wasm_add(message: Json<AddRequest>) -> JsonValue {
    let message = message.0;
    let rv = add(message);
    json!({
        "result": rv,
    })
}

#[post("/wasm/llvm_add", format = "json", data = "<message>")]
fn wasm_llvm_add(message: Json<AddRequest>) -> JsonValue {
    let message = message.0;
    let rv = llvm_add(message);
    json!({
        "result": rv,
    })
}

#[post("/native/base64", format = "json", data = "<message>")]
fn native_base64(message: Json<Base64Request>) -> JsonValue {
    let message = message.0;
    let rv = encode(&message.text);
    json!({
        "text": rv,
    })
}

#[post("/wasm/base64", format = "json", data = "<message>")]
fn wasm_base64(message: Json<Base64Request>) -> JsonValue {
    let message = message.0;
    let rv = base64(message);
    json!({
        "text": rv,
    })
}

#[post("/wasm/llvm_base64", format = "json", data = "<message>")]
fn wasm_llvm_base64(message: Json<Base64Request>) -> JsonValue {
    let message = message.0;
    let rv = llvm_base64(message);
    json!({
        "text": rv,
    })
}

fn start_server() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                native_add,
                wasm_add,
                wasm_llvm_add,
                native_base64,
                wasm_base64,
                wasm_llvm_base64,
            ],
        )
        .launch();
}

fn main() {
    start_server();
}
