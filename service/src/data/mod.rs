use crate::backend;
use crate::errors::{FinError, ResultFin};
use crate::models;
use crate::server;
use chrono::prelude::*;
use std::collections::HashMap;

use r2d2;

pub trait FinDb {
    //========== TASKS
    fn get_incomplete_tasks(&self) -> ResultFin<Vec<models::Item>>;

    fn get_incomplete_by_proj_id(
        &self,
        proj_id: i64,
    ) -> ResultFin<Vec<models::Item>>;

    fn get_all_tasks(&self) -> ResultFin<Vec<models::Item>>;

    //========== TASKS
    fn get_all_projects(&self) -> ResultFin<Vec<models::Project>>;
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
            .prep_exec(
                "SELECT itemId, title, description, projectId, deadlineDate, memberId FROM taskfreak.frk_item
                WHERE itemId NOT IN (
                    SELECT DISTINCT itemId FROM taskfreak.frk_itemStatus WHERE statusKey = 5
                ) ORDER BY deadlineDate",
                ()
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(row_to_item)
                    .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Item>`
            })
            .map_err(|err| {
                lineError!(self.logger, err);
                FinError::DatabaseErr
            });

        items
    }

    fn get_incomplete_by_proj_id(
        &self,
        proj_id: i64,
    ) -> ResultFin<Vec<models::Item>> {
        self.conn
            .prep_exec(
                "SELECT itemId, title, description, projectId, deadlineDate, memberId FROM frk_item
                WHERE projectId = :a ORDER BY deadlineDate",
                params!{"a" => proj_id},
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(row_to_item)
                    .collect()
            })
            .map_err(|err| {
                lineError!(self.logger, err);
                FinError::DatabaseErr
            })
    }

    fn get_all_tasks(&self) -> ResultFin<Vec<models::Item>> {
        self.conn
            .prep_exec(
                "SELECT itemId, title, description, projectId, deadlineDate, memberId FROM frk_item
                ORDER BY deadlineDate",
                (),
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(row_to_item)
                    .collect() // Collect payments so now `QueryResult` is mapped to `Vec<Item>`
            })
            .map_err(|err| {
                lineError!(self.logger, err);
                FinError::DatabaseErr
            })
    }

    fn get_all_projects(&self) -> ResultFin<Vec<models::Project>> {
        self.conn
            .prep_exec(
                "SELECT projectId, name, description FROM frk_project",
                (),
            )
            .map(|result| {
                result
                    .map(|x| x.unwrap())
                    .map(|row| {
                        let (projectId, name, description) =
                            mysql::from_row(row);
                        models::Project::new(projectId, name, description)
                    })
                    .collect()
            })
            .map_err(|err| {
                lineError!(self.logger, err);
                FinError::DatabaseErr
            })
    }
}

fn row_to_item(row: mysql::Row) -> models::Item {
    let itemId = row.get(0).unwrap();
    let title = row.get(1).unwrap();
    let description = row.get(2).unwrap();
    let projectId = row.get(3).unwrap();
    let deadlineDate = match row.get_opt(4).unwrap() {
        Ok(d) => d,
        Err(err) => {
            dbg!(err);
            NaiveDate::from_ymd(0000, 1, 1)
        }
    };
    let memberId = row.get(5).unwrap();
    models::Item::new(
        itemId,
        title,
        description,
        projectId,
        deadlineDate,
        memberId,
    )
}
