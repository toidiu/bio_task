mod db_types;

use crate::backend;
use crate::errors::{FinError, ResultFin};
use crate::server;
use chrono::prelude::*;
use std::collections::HashMap;

use r2d2;

pub(crate) use self::db_types::*;

pub struct PgFinDb {
    // pub conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
    logger: slog::Logger,
}

impl PgFinDb {
    pub fn new(
        // conn: r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>,
        logger: slog::Logger,
    ) -> Self {
        PgFinDb {
            // conn: conn,
            logger: logger.new(o!("mod" => "data")),
        }
    }
}

pub trait FinDb {
    //========== USER
    fn get_user(&self, email: &str) -> ResultFin<db_types::UserData>;
}

impl FinDb for PgFinDb {
    fn get_user(&self, email: &str) -> ResultFin<db_types::UserData> {
        unimplemented!()
    }
}
