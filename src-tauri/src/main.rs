#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
extern crate lm_lib;

use std::vec;

use lm_lib::{
    api::*,
    models::{FriendList, FriendTab, LoginResponse, Sex, User},
    orm::Db,
};
use tauri::{command, State};

/// login to server, if success, should got a ws connection.
#[command]
async fn sign_in(
    username: String,
    password: String,
    database: State<'_, Db>,
) -> Result<LoginResponse, String> {
    login(username, password, database).await
}

#[command]
async fn show_friend_list(name: &str) -> Result<Vec<FriendList>, ()> {
    query_friend_list(name).await
}

#[command]
async fn show_friend_tabs() -> Vec<FriendTab> {
    query_friend_tabs().await
}

fn main() {
    // let _ = fix_path_env::fix();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            sign_in,
            show_friend_list,
            show_friend_tabs
        ])
        .manage(Db(init_db()))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_db() -> Vec<User> {
    let mut users = Vec::new();
    let u1 = User::new("lm", "lm", "lm@gmail.com", Sex::Female);
    let u2 = User::new("yaphets", "yaphets", "yaphets@gmail.com", Sex::Male);
    let u3 = User::new("sdy", "sdy", "sdy@gmail.com", Sex::Female);
    users.push(u1);
    users.push(u2);
    users.push(u3);
    users
}
