mod db_types;

use crate::backend;
use crate::errors::{FinError, ResultFin};
use crate::models;
use crate::server;
use chrono::prelude::*;
use std::collections::HashMap;

use r2d2;

pub(crate) use self::db_types::*;

pub trait FinDb {
    //========== USER
    fn get_incomplete_tasks(&self) -> ResultFin<Vec<models::Item>>;
}

pub struct PgFinDb {
    pub conn: mysql::Pool,
    logger: slog::Logger,
}

impl PgFinDb {
    pub fn new(conn: mysql::Pool, logger: slog::Logger) -> Self {
        PgFinDb {
            conn: conn,
            logger: logger.new(o!("mod" => "data")),
        }
    }
}

impl FinDb for PgFinDb {
    fn get_incomplete_tasks(&self) -> ResultFin<Vec<models::Item>> {
        let items: ResultFin<Vec<models::Item>> = self
            .conn
            .prep_exec("SELECT itemId, title, description from frk_item", ())
            //     .conn
            //     .prep_exec("SELECT memberId, email from frk_member", ())
            .map(|result| {
                //         // In this closure we will map `QueryResult` to `Vec<Item>`
                //         // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
                //         // will map each `MyResult` to contained `row` (no proper error handling)
                //         // and second call to `map` will map each `row` to `Item`
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        // Note that from_row will panic if you don't follow your schema
                        let (itemId, title, description) = mysql::from_row(row);
                        models::Item::new(itemId, title, description)
                    })
                    .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Item>`
            })
            .map_err(|err| FinError::DatabaseErr);

        // println!("{:?}", items);
        // println!("here");
        items
    }
}
