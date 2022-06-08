use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    device_id: String,
    user_id: Option<String>,
}

impl Session {
    pub fn new(device_id: String) -> Session {
        Session {device_id: device_id, user_id: None}
    }
}
