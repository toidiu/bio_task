use crate::data;
use crate::errors::*;
use crate::models;
use crate::server;
use crate::std_ext::ExtIterator;
use chrono::prelude::*;
use std::collections::HashMap;

pub trait Backend {
    fn get_incomplete_tasks(&self) -> ResultFin<Vec<models::Item>>;

    fn get_incomplete_by_proj_id(
        &self,
        proj_id: i64,
    ) -> ResultFin<Vec<models::Item>>;

    fn get_all_tasks(&self) -> ResultFin<Vec<models::Item>>;

    fn get_all_projects(&self) -> ResultFin<Vec<models::Project>>;
}

impl Backend {
    pub fn get_logger_context(logger: slog::Logger) -> slog::Logger {
        logger.new(o!("mod" => "portfolio_backend"))
    }
}

pub struct DefaultBackend<T: data::FinDb> {
    db: T,
    logger: slog::Logger,
}

impl<T: data::FinDb> DefaultBackend<T> {
    pub fn new(db: T, logger: slog::Logger) -> DefaultBackend<T> {
        DefaultBackend {
            db: db,
            logger: Backend::get_logger_context(logger),
        }
    }
}

impl<T: data::FinDb> Backend for DefaultBackend<T> {
    fn get_incomplete_tasks(&self) -> ResultFin<Vec<models::Item>> {
        self.db.get_incomplete_tasks()
    }

    fn get_incomplete_by_proj_id(
        &self,
        proj_id: i64,
    ) -> ResultFin<Vec<models::Item>> {
        self.db.get_incomplete_by_proj_id(proj_id)
    }

    fn get_all_tasks(&self) -> ResultFin<Vec<models::Item>> {
        self.db.get_all_tasks()
    }

    fn get_all_projects(&self) -> ResultFin<Vec<models::Project>> {
        self.db.get_all_projects()
    }
}
