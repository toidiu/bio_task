use crate::server;
use chrono::prelude::*;
use std::collections::HashMap;

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

#[derive(Debug)]
pub struct DomainTicKind(String);

#[derive(Debug)]
pub struct TickerData {
    pub id: i64,
    pub symbol: String,
    pub fk_exchange: i32,
    pub fee: f32,
    pub kind: DomainTicKind,
}
