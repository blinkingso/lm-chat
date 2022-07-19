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
            "apple",
            "Apple",
            "http://localhost:8080/images/avatar.jpeg",
        )]),
        "saved_groups" => Ok(vec![
            FriendList::with_saved_groups(
                "ali",
                "Ali Cloud",
                "http://localhost:8080/images/avatar.jpeg",
            ),
            FriendList::with_saved_groups(
                "google",
                "Google Cloud",
                "http://localhost:8080/images/avatar.jpeg",
            ),
        ]),
        "official_accounts" => Ok(vec![FriendList::with_official_accounts(
            "ali",
            "Ali Cloud",
            "http://localhost:8080/images/avatar.jpeg",
        )]),
        "contacts" => Ok(vec![
            FriendList::with_contacts("lm", "Lm", "http://localhost:8080/images/avatar.jpeg"),
            FriendList::with_contacts("sdy", "Sdy", "http://localhost:8080/images/avatar.jpeg"),
            FriendList::with_contacts("gm", "Gm", "http://localhost:8080/images/avatar.jpeg"),
            FriendList::with_contacts("lily", "Lily", "http://localhost:8080/images/avatar.jpeg"),
            FriendList::with_contacts("lily1", "Lily1", "http://localhost:8080/images/avatar.jpeg"),
            FriendList::with_contacts("lily2", "Lily2", "http://localhost:8080/images/avatar.jpeg"),
            FriendList::with_contacts("lily3", "Lily3", "http://localhost:8080/images/avatar.jpeg"),
            FriendList::with_contacts("lily4", "Lily4", "http://localhost:8080/images/avatar.jpeg"),
        ]),
        _ => Err(()),
    }
}
pub async fn query_friend_tabs() -> Vec<FriendTab> {
    vec![
        FriendTab::new("new_friends", "New Friends", 0),
        FriendTab::new(
            "saved_groups",
            "Saved Groups",
            query_tab_list_counts("saved_groups").await,
        ),
        FriendTab::new(
            "official_accounts",
            "Official Accounts",
            query_tab_list_counts("official_accounts").await,
        ),
        FriendTab::new(
            "contacts",
            "Contacts",
            query_tab_list_counts("contacts").await,
        ),
    ]
}

async fn query_tab_list_counts(name: &str) -> usize {
    if let Ok(v) = query_friend_list(name).await {
        v.len()
    } else {
        0
    }
}
