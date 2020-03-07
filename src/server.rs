use rocket::data::Data;
use rocket_contrib::json::JsonValue;
use serde::{Deserialize, Serialize};
use wasmer_runtime::{imports, instantiate, Func};

use crate::wasmer::{get_request, post_request};

#[get("/<name>")]
fn process_get(name: String) -> JsonValue {
    let rv = get_request(name);
    json!({
        "result": rv,
    })
}

#[post("/<name>", data = "<data>")]
fn process_post(name: String, data: Data) -> JsonValue {
    let rv = post_request(name, data);
    json!({
        "result": rv,
    })
}

pub fn start_server() {
    rocket::ignite()
        .mount("/", routes![process_get, process_post])
        .launch();
}
