use crate::data;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TasksResp {
    pub name: String,
}
