#![allow(dead_code)]

use crate::models::LoginResponse;
use crate::orm::Db;
use regex::Regex;
use tauri::{window::Window, State};

use lazy_static::lazy_static;

lazy_static! {
    static ref REGEX_EMAIL: Regex =
        Regex::new(r"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$")
            .unwrap();
}

pub async fn login(
    window: Window,
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
            return Ok(lp);
        } else {
            return Err(String::from("your password is wrong!"));
        }
    } else {
        Err(format!("user {} not found.", username))
    }
}
