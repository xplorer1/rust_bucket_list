use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub user_id: i32,
    pub name: String,
    pub password: String,
    pub email: String,
}