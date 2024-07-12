use serde::{Deserialize, Serialize};
use spin_sdk::http::conversions::IntoBody;

#[derive(Default, Serialize, Deserialize)]
pub struct Counter {
    pub count: i32,
}

impl IntoBody for Counter {
    fn into_body(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap_or_default()
    }
}

pub const COUNTER_KEY: &str = "spin_counter";
