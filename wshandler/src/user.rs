#[derive(Debug)]
pub(crate) struct User {
    uid: i64,
    dev: String,
    user_key: Option<String>,
}

impl User {
    pub fn get_user_key(&self) -> Option<String> {
        if self.user_key.is_some() {
            return self.user_key.clone();
        }

        if self.uid < 1 {
            return None;
        }

        Some(format!("{} {}", self.uid, self.dev))
    }
}
