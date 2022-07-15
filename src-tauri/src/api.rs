#![allow(dead_code)]

use crate::models::{FriendList, FriendTab, LoginResponse};
use crate::orm::Db;
use regex::Regex;
use tauri::State;

use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_EMAIL: Regex =
        Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
            .unwrap();
}

pub async fn login(
    username: String,
    password: String,
    db: State<'_, Db>,
) -> Result<LoginResponse, String> {
    let db = db.inner();
    let user = if REGEX_EMAIL.is_match(username.as_str()) {
        db.query_user_by_email(username.as_str())
    } else {
        db.query_user_by_uname(username.as_str())
    };
    if let Some(user) = user {
        if password.is_empty() || password.trim().is_empty() {
            return Err(String::from("your password is empty"));
        }
        if password.eq(user.passwd.as_str()) {
            let lp = LoginResponse {
                user: user.clone(),
                token: String::from("token"),
                expire_time: 30u64,
            };
            Ok(lp)
        } else {
            Err(String::from("your password is wrong!"))
        }
    } else {
        Err(format!("user {} not found.", username))
    }
}

pub async fn query_friend_list(name: &str) -> Result<Vec<FriendList>, ()> {
    match name {
        "new_friends" => Ok(vec![FriendList::with_new_friends(
            "Apple",
            "../assets/avatar.jpeg",
        )]),
        "saved_groups" => Ok(vec![
            FriendList::with_saved_groups("Ali Cloud", "../assets/avatar.jpeg"),
            FriendList::with_saved_groups("Google Cloud", "../assets/avatar.jpeg"),
        ]),
        "official_accounts" => Ok(vec![FriendList::with_official_accounts(
            "Ali Cloud",
            "../assets/avatar.jpeg",
        )]),
        "contacts" => Ok(vec![
            FriendList::with_contacts("Lm", "../assets/avatar.jpeg"),
            FriendList::with_contacts("Sdy", "../assets/avatar.jpeg"),
            FriendList::with_contacts("Gm", "../assets/avatar.jpeg"),
            FriendList::with_contacts("Lily", "../assets/avatar.jpeg"),
        ]),
        _ => Err(()),
    }
}
pub async fn query_friend_tabs() -> Vec<FriendTab> {
    vec![
        FriendTab::new("new_friends", "New Friends"),
        FriendTab::new("saved_groups", "Saved Groups"),
        FriendTab::new("official_accounts", "Official Accounts"),
        FriendTab::new("contacts", "Contacts"),
    ]
}
