use rocket_contrib::json::{Json, JsonValue};

use crate::wasmer::b64encode;

#[derive(Deserialize)]
struct OutgoingRequest {
    text: String,
    keyword: String,
    username: String,
}

#[post("/labs/wasm/outgoing", format = "json", data = "<message>")]
fn outgoing(message: Json<OutgoingRequest>) -> JsonValue {
    let message = message.0;
    let raw_text = message.text;
    let kw = message.keyword;
    let (_, text) = raw_text.split_at(kw.len() + 1);
    let encoded = b64encode(text);
    json!({
        "text": format!("@{}, Base64.encode(\"{}\") = `{}`",
                        message.username,
                        text,
                        encoded),
    })
}

pub fn start_server() {
    rocket::ignite().mount("/", routes![outgoing]).launch();
}
