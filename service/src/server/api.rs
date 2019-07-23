use crate::data;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TasksResp {
    pub name: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserDataApi {
    pub id: i64,
    pub email: String,
}
