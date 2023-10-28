use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WSMessage {
    group: String,
    uid: i64,
    pub data: String,
}

impl WSMessage {
    pub fn new(group: &str, uid: i64, data: &str) -> Self {
        WSMessage {
            group: group.to_string(),
            uid,
            data: data.to_string(),
        }
    }
}
