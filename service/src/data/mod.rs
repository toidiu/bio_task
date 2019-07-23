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
    fn get_user(&self, email: &str) -> ResultFin<db_types::UserData>;

    fn get_user_with_pass(
        &self,
        email: &str,
    ) -> ResultFin<db_types::UserDataWithPass>;

    fn does_user_exist(&self, email: &str) -> ResultFin<bool>;

    //========== TASKS
    fn get_incomplete_tasks(&self) -> ResultFin<Vec<models::Item>>;

    //========== PROJECTS
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
    fn get_user(&self, email: &str) -> ResultFin<db_types::UserData> {
        // // table users
        // let stmt = &format!(
        //     "SELECT {} FROM {} WHERE email = $1",
        //     &db_types::UserData::sql_fields(),
        //     &db_types::UserData::sql_table(),
        // );

        // let rows = &self.conn.query(stmt, &[&email]).map_err(|err| {
        //     lineError!(self.logger, err);
        //     err
        // })?;

        // rows.iter()
        //     .next()
        //     .map(|row| {
        //         db_types::UserData::from_postgres_row(row).map_err(|err| {
        //             lineError!(self.logger, err);
        //             FinError::DatabaseErr
        //         })
        //     })
        //     .ok_or(FinError::DatabaseErr)?
        unimplemented!()
    }

    fn get_user_with_pass(
        &self,
        email: &str,
    ) -> ResultFin<db_types::UserDataWithPass> {
        // // table users
        // let stmt = &format!(
        //     "SELECT {} FROM {} WHERE email = $1",
        //     &db_types::UserDataWithPass::sql_fields(),
        //     &db_types::UserDataWithPass::sql_table(),
        // );

        // let rows = &self.conn.query(stmt, &[&email]).map_err(|err| {
        //     lineError!(self.logger, err);
        //     err
        // })?;

        // rows.iter()
        //     .next()
        //     .map(|row| {
        //         db_types::UserDataWithPass::from_postgres_row(row).map_err(
        //             |err| {
        //                 lineError!(self.logger, err);
        //                 FinError::DatabaseErr
        //             },
        //         )
        //     })
        //     .ok_or(FinError::DatabaseErr)?
        unimplemented!()
    }

    fn does_user_exist(&self, email: &str) -> ResultFin<bool> {
        // // table users
        // let stmt = &format!(
        //     "SELECT 1 from {} where email=$1",
        //     &db_types::UserData::sql_table(),
        // );

        // let rows = self.conn.query(stmt, &[&email]);

        // let rows = rows.map_err(|err| {
        //     lineError!(self.logger, err);
        //     err
        // })?;

        // let ret = !rows.is_empty();
        // Ok(ret)
        unimplemented!()
    }

    fn get_incomplete_tasks(&self) -> ResultFin<Vec<models::Item>> {
        let items: ResultFin<Vec<models::Item>> = self
            .conn
            .prep_exec(
                "SELECT it.itemId, it.title, it.description, max(iss.statusKey),
                    it.deadlineDate, mem.firstname FROM frk_item it
                JOIN frk_member mem ON (it.memberId = mem.memberId)
                JOIN frk_itemStatus iss ON (it.itemId = iss.itemId)
                WHERE it.itemId NOT IN
                    (SELECT DISTINCT itemId FROM frk_itemStatus WHERE statusKey = 5)
                GROUP BY it.itemId
                ORDER BY deadlineDate",
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
    let statusKey = row.get(3).unwrap();
    let deadlineDate = match row.get_opt(4).unwrap() {
        Ok(d) => d,
        Err(err) => {
            dbg!(err);
            NaiveDate::from_ymd(9999, 1, 1)
        }
    };
    let memberId = row.get(5).unwrap();
    models::Item::new(
        itemId,
        title,
        description,
        statusKey,
        deadlineDate,
        memberId,
    )
}
