use crate::data;
use crate::errors::*;
use crate::global::CONFIG;
use crate::server;
use crate::std_ext::ExtIterator;
use chrono::prelude::*;
use std::collections::HashMap;

pub trait TasksBackend {
    fn get_tickers(&self, ids: &Vec<i64>) -> HashMap<i64, i64>;
}

impl TasksBackend {
    pub fn get_logger_context(logger: slog::Logger) -> slog::Logger {
        logger.new(o!("mod" => "portfolio_backend"))
    }
}

pub struct DefaultTasksBackend<T: data::FinDb> {
    db: T,
    logger: slog::Logger,
}

impl<T: data::FinDb> DefaultTasksBackend<T> {
    pub fn new(db: T, logger: slog::Logger) -> DefaultTasksBackend<T> {
        DefaultTasksBackend {
            db: db,
            logger: TasksBackend::get_logger_context(logger),
        }
    }
}

impl<T: data::FinDb> TasksBackend for DefaultTasksBackend<T> {
    fn get_tickers(&self, ids: &Vec<i64>) -> HashMap<i64, i64> {
        // // Let's select payments from database
        // let selected_payments: Vec<Member> = conn
        //     .prep_exec("SELECT memberId, email from frk_member", ())
        //     .map(|result| {
        //         // In this closure we will map `QueryResult` to `Vec<Payment>`
        //         // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        //         // will map each `MyResult` to contained `row` (no proper error handling)
        //         // and second call to `map` will map each `row` to `Payment`
        //         result
        //             .map(|x| x.unwrap())
        //             .map(|row| {
        //                 // ⚠️ Note that from_row will panic if you don't follow your schema
        //                 let (member_id, email) = mysql::from_row(row);
        //                 Member {
        //                     memberId: member_id,
        //                     email: email,
        //                 }
        //             })
        //             .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
        //     })
        //     .expect("asdfasd"); // Unwrap `Vec<Payment>`

        // println!("{:?}", selected_payments);
        // println!("here");
        unimplemented!()
    }
}
