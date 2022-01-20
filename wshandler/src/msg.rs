use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct WSMessage {
    group: String,
    uid: i64,
    data: String,
}

impl WSMessage{
    fn new(group: &str, uid: i64, data: &str)-> Self{
        WSMessage{
            group: group.to_string(),
            uid,
            data: data.to_string(),
        }
    }
}