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
        unimplemented!()
    }
}
