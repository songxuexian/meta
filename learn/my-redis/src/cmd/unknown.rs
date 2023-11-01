#[derive(Debug)]
pub struct Unknown {
    command_name: String,
}

impl Unknown {
    pub fn new(key: impl ToString) -> Unknown {
        Self {
            command_name: key.to_string(),
        }
    }
}
