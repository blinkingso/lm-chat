use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
    pub id: u64,
    pub chat_id: String,
    pub passwd: String,
    pub email: String,
    pub phone: Option<String>,
    pub name: String,
    pub profile_image: Option<Vec<u8>>,
    pub sex: i32,
    pub region: String,
    pub personalized_signature: Option<String>,
}
