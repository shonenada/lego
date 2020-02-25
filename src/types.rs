use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OutgoingRequest {
    pub text: String,
    pub keyword: String,
    pub username: String,
}
