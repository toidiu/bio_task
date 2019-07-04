use crate::data;
use crate::errors::*;
use crate::global::CONFIG;
use crate::models;
use crate::server;
use crate::std_ext::ExtIterator;
use chrono::prelude::*;
use std::collections::HashMap;

pub trait TasksBackend {
    fn get_incomplete_by_proj_id(
        &self,
        proj_id: i64,
    ) -> ResultFin<Vec<models::Item>>;

    fn get_all_tasks(&self) -> ResultFin<Vec<models::Item>>;
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
    fn get_incomplete_by_proj_id(
        &self,
        proj_id: i64,
    ) -> ResultFin<Vec<models::Item>> {
        self.db.get_incomplete_by_proj_id(proj_id)
    }

    fn get_all_tasks(&self) -> ResultFin<Vec<models::Item>> {
        self.db.get_all_tasks()
    }
}
