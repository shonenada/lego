use rocket_contrib::json::{Json, JsonValue};

use crate::types::OutgoingRequest;
use crate::wasmer::process_outgoing;

#[post("/labs/wasm/outgoing/<name>", format = "json", data = "<message>")]
fn outgoing(name: String, message: Json<OutgoingRequest>) -> JsonValue {
    let message = message.0;
    let rv = process_outgoing(name, message);
    json!({
        "text": rv,
    })
}

pub fn start_server() {
    rocket::ignite().mount("/", routes![outgoing]).launch();
}
