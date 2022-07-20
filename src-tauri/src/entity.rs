use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[repr(C)]
pub enum Sex {
    #[default]
    Male = 0,
    Female = 1,
}

impl From<i32> for Sex {
    fn from(sex: i32) -> Self {
        match sex {
            1 => Self::Female,
            _ => Self::Male,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExtendsInfo {
    pub comment: Option<String>,
}

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
    pub profile_image: Option<Vec<u8>>,
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
            region: String::from("China Mainland"),
            ..Default::default()
        }
    }
}

impl From<models::User> for User {
    fn from(user: models::User) -> Self {
        Self {
            id: user.id,
            chat_id: user.chat_id,
            passwd: user.passwd,
            phone: user.phone,
            email: user.email,
            name: user.name,
            profile_image: user.profile_image,
            sex: Sex::from(user.sex),
            region: user.region,
            personalized_signature: user.personalized_signature,
            extends: None,
        }
    }
}

impl From<User> for models::User {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            chat_id: user.chat_id,
            passwd: user.passwd,
            phone: user.phone,
            email: user.email,
            name: user.name,
            profile_image: user.profile_image,
            sex: user.sex as i32,
            region: user.region,
            personalized_signature: user.personalized_signature,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
    pub expire_time: u64,
}

#[derive(Debug, Serialize)]
pub struct FriendTab {
    pub id: u32,
    pub name: String,
    pub show_name: String,
    pub count: usize,
}

impl FriendTab {
    pub fn new(name: &str, show_name: &str, count: usize) -> Self {
        FriendTab {
            id: 0,
            name: name.to_string(),
            show_name: show_name.to_string(),
            count,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FriendList {
    pub first_letter: char,
    pub name: String,
    pub r#type: u8,
    // if unset default to unique name
    pub nick_name: String,
    // avatar
    pub avatar: String,
}

impl FriendList {
    fn new(ty: u8, name: &str, nick_name: &str, avatar: &str) -> Self {
        // calculate first letter.
        let first_letter = if let Some(ch) = nick_name.chars().next() {
            ch
        } else {
            '#'
        };

        FriendList {
            first_letter,
            name: name.to_string(),
            r#type: ty,
            nick_name: nick_name.to_string(),
            avatar: avatar.to_string(),
        }
    }

    pub fn with_new_friends(name: &str, nick_name: &str, avatar: &str) -> Self {
        Self::new(0, name, nick_name, avatar)
    }
    pub fn with_saved_groups(name: &str, nick_name: &str, avatar: &str) -> Self {
        Self::new(1, name, nick_name, avatar)
    }
    pub fn with_official_accounts(name: &str, nick_name: &str, avatar: &str) -> Self {
        Self::new(2, name, nick_name, avatar)
    }
    pub fn with_contacts(name: &str, nick_name: &str, avatar: &str) -> Self {
        Self::new(3, name, nick_name, avatar)
    }
}
