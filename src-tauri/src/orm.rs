use crate::models::User;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Db(pub Vec<User>);

impl Db {
    pub fn query_user_by_uname<'a>(&self, uname: &'a str) -> Option<&User> {
        self.0
            .iter()
            .find(|u| u.chat_id.eq_ignore_ascii_case(uname))
    }

    pub fn query_user_by_email<'a>(&self, email: &'a str) -> Option<&User> {
        self.0.iter().find(|u| u.email.eq_ignore_ascii_case(email))
    }
}
