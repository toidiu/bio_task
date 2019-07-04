use crate::backend;
use crate::data;
use crate::errors::{ErrMessage, FinError};
use crate::global::{CONFIG, ROOT};

use http::StatusCode;
use mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;
use std::env;
use std::sync::Arc;
use std::thread;

use warp::{Filter, Rejection};

mod api;
mod auth;
mod tasks_server;

pub use api::*;
pub use auth::UserId;

lazy_static! {
    static ref CONNECTION: Arc<r2d2::Pool<r2d2_mysql::MysqlConnectionManager>> = {
        let db_url =
            "mysql://rusty:6VO3SaW3PwMBTcyK@192.168.2.100:3306/taskfreak".to_string();
        let opts = Opts::from_url(&db_url).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MysqlConnectionManager::new(builder);
        // r2d2::Pool::builder()
        //     .max_size(CONFIG.database.pool_size)
        //     .build(manager)
        //     .expect("Failed to create pool")
        Arc::new(r2d2::Pool::builder().max_size(4).build(manager).unwrap())
    };
    // logger
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "server"));
}

#[derive(Debug, PartialEq, Eq)]
pub struct Member {
    memberId: i64,
    email: String,
}

pub fn start_server() {
    println!("Listening on http://localhost:{}", CONFIG.app.port);
    lineInfo!(
        LOGGER,
        format!("Listening on http://localhost:{}", CONFIG.app.port)
    );

    // HEADERS
    let with_cors = warp::cors()
        .allow_origin(CONFIG.app.cors_origin.as_str())
        .allow_credentials(true)
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    let mut conn = CONNECTION.clone().get().unwrap();

    // let with_task_backend = {
    //     warp::any().map(|| match CONNECTION.clone().get() {
    //         Ok(conn) => Ok(backend::DefaultTasksBackend::new(
    //             data::PgFinDb::new(
    //                 // conn,
    //                 backend::TasksBackend::get_logger_context(
    //                     (*LOGGER).clone(),
    //                 ),
    //             ),
    //             (*LOGGER).clone(),
    //         )),
    //         Err(err) => {
    //             error!(LOGGER, "{}: {}", line!(), err);
    //             Err(warp::reject::custom(FinError::DatabaseErr))
    //         }
    //     })
    // };

    // // TASKS===============
    // let task_path = warp::path("tasks");
    // // GET -> tasks/incomplete
    // //     get incomplete (sort by date)
    // let get_incomplete_task = warp::get2()
    //     .and(task_path)
    //     .and(warp::path("incomplete"))
    //     .and(warp::path::end())
    //     // .and(with_task_backend)
    //     .and_then(tasks_server::get_tasks);

    // // POST -> tasks
    // let create_task = warp::post2()
    //     .and(task_path)
    //     .and(warp::path::end())
    //     // .and(warp::body::json())
    //     // .and(with_task_backend)
    //     .and_then(tasks_server::create_task);

    // let task_api = get_incomplete_task.or(create_task);

    // // DEPENDENCY===============
    // //     post create dependency
    // //     get dependency by task id

    // // combine apis
    // let api = task_api;

    // let routes = api.recover(recover_error).with(with_cors);
    // warp::serve(routes).run(([127, 0, 0, 1], CONFIG.app.port));
}

fn recover_error(err: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if let Some(&err) = err.find_cause::<FinError>() {
        let status_code = match err {
            FinError::NotLoggedIn => StatusCode::UNAUTHORIZED,
            FinError::BadRequestErr => StatusCode::BAD_REQUEST,
            FinError::NotFoundErr => StatusCode::NOT_FOUND,
            FinError::DatabaseErr | FinError::ServerErr => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let json = warp::reply::json(&err.to_msg());
        Ok(warp::reply::with_status(json, status_code))
    } else {
        let status_code = StatusCode::NOT_FOUND;
        let json = warp::reply::json(&ErrMessage::new(
            status_code,
            "not found".to_string(),
        ));
        Ok(warp::reply::with_status(json, status_code))
    }
}
