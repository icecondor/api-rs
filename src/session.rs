use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Session {
    #[serde(skip)]
    pub id: String,
    pub device_id: String,
    pub user_id: String,
}

impl Session {
    pub fn new(device_id: String, user_id: String) -> Session {
        Session {
            id: uuid::Uuid::new_v4().to_string(),
            device_id: device_id,
            user_id: user_id,
        }
    }
}
