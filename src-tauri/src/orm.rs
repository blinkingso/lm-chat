use crate::entity::User;
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

    // simulate db operation.
}

// #[cfg(feature = "server")]
pub mod server {
    use diesel::prelude::*;
    use diesel::sqlite::SqliteConnection;
    use dotenv::dotenv;
    use std::env;

    pub fn establish_connection() -> SqliteConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }

    #[test]
    fn test_add_user() {
        use crate::schema::users::dsl::*;
        use crate::{entity::*, models};
        use diesel::query_dsl::RunQueryDsl;
        let conn = establish_connection();
        let user = User::new("yaphets", "123456", "714232542@qq.com", Sex::Male);
        let user2 = User::new("lm", "123456", "lm@qq.com", Sex::Female);
        let mut u1 = models::CreateUser::from(user);
        let mut u2 = models::CreateUser::from(user2);
        u1.name = String::from("Yaphets");
        u2.name = String::from("LuMen");
        if let Err(e) = diesel::insert_into(users).values(&u2).execute(&conn) {
            eprintln!("Failed to create user for : {:?}", e);
        }

        let results = users
            .limit(5)
            .load::<models::User>(&conn)
            .expect("Error loading users");
        for u in results {
            println!("{:?}", u);
        }
    }
}
