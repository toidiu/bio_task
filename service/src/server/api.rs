use crate::data;
use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PortfolioStateResp {
    pub name: String,
}
