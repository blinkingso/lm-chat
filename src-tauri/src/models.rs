use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[repr(C)]
pub enum Sex {
    #[default]
    Male = 0,
    Female = 1,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtendsInfo {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// User Model
pub struct User {
    pub id: u64,
    // unique chat_id generate by system or change it once a year that not exists.
    pub chat_id: String,
    pub passwd: String,
    pub phone: Option<String>,
    pub email: String,
    pub name: String,
    pub profile_image: Option<String>,
    pub sex: Sex,
    pub region: String,
    pub personalized_signature: Option<String>,
    pub extends: Option<ExtendsInfo>,
}

impl User {
    pub fn new(chat_id: &str, passwd: &str, email: &str, sex: Sex) -> Self {
        Self {
            chat_id: chat_id.to_string(),
            email: email.to_string(),
            passwd: passwd.to_string(),
            name: chat_id.to_string(),
            sex,
            region: String::from("China"),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
    pub expire_time: u64,
}
