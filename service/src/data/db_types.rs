#[derive(Serialize, Deserialize, Debug)]
pub struct UserDataWithPass {
    pub id: i64,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub id: i64,
    pub email: String,
}
