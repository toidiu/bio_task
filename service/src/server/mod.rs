use crate::backend;
use crate::data;
use crate::errors::{ErrMessage, FinError};
use crate::global::ROOT;

use http::StatusCode;
use mysql::{Opts, OptsBuilder};
use r2d2_mysql::MysqlConnectionManager;
use std::env;
use std::sync::Arc;
use std::thread;

use warp::{Filter, Rejection};

mod api;
mod tasks_server;

pub use api::*;

lazy_static! {
    // logger
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "server"));
}

pub fn start_server() {
    println!("listening on: http://localhost:8000");
    // HEADERS
    let with_cors = warp::cors()
        .allow_origin("http://localhost:1234")
        .allow_credentials(true)
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    let with_task_backend = {
        warp::any().map(|| {
            let pool = mysql::Pool::new(
                "mysql://rusty:6VO3SaW3PwMBTcyK@localhost:3306/taskfreak",
            )
            .unwrap();
            Ok(backend::DefaultTasksBackend::new(
                data::PgFinDb::new(
                    pool,
                    backend::TasksBackend::get_logger_context(
                        (*LOGGER).clone(),
                    ),
                ),
                (*LOGGER).clone(),
            ))
        })
    };

    // TASKS===============
    let task_path = warp::path("tasks");
    // GET -> tasks/incomplete/project/:id
    let get_incomplete_tasks = warp::get2()
        .and(task_path)
        .and(warp::path("incomplete"))
        .and(warp::path::end())
        .and(with_task_backend)
        .and_then(tasks_server::get_incomplete_tasks);

    // GET -> tasks/incomplete/project/:id
    let get_incomplete_by_proj_id = warp::get2()
        .and(task_path)
        .and(warp::path("incomplete"))
        .and(warp::path("project"))
        .and(warp::path::param2::<i64>())
        .and(warp::path::end())
        .and(with_task_backend)
        .and_then(tasks_server::get_incomplete_by_proj_id);

    let get_all_tasks = warp::get2()
        .and(task_path)
        .and(warp::path::end())
        .and(with_task_backend)
        .and_then(tasks_server::get_all_tasks);

    // POST -> tasks
    let create_task = warp::post2()
        .and(task_path)
        .and(warp::path::end())
        // .and(warp::body::json())
        .and(with_task_backend)
        .and_then(tasks_server::create_task);

    let task_api = get_incomplete_by_proj_id
        .or(get_incomplete_tasks)
        .or(get_all_tasks)
        .or(create_task);

    // DEPENDENCY===============
    //     post create dependency
    //     get dependency by task id

    // combine apis
    let api = task_api;

    let routes = api.recover(recover_error).with(with_cors);
    warp::serve(routes).run(([127, 0, 0, 1], 8000));
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
