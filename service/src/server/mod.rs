use crate::backend;
use crate::data;
use crate::errors::{ErrMessage, FinError};
use crate::global::ROOT;

use http::StatusCode;
use mysql::{Opts, OptsBuilder};
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::sync::Arc;
use std::thread;
use toml::Value;

use warp::{Filter, Rejection};

mod api;
mod auth;
mod tasks_server;
mod user_server;

pub use api::*;

lazy_static! {
    // logger
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "server"));
    static ref CONNECTION: mysql::Pool = {
            mysql::Pool::new(MY_SQL_URL).unwrap()
    };
}

const MY_SQL_URL: &str =
    "mysql://rusty:6VO3SaW3PwMBTcyK@localhost:3306/taskfreak";

pub fn start_server() {
    println!("listening on: http://localhost:8000");
    // HEADERS
    let with_cors = warp::cors()
        .allow_origin("http://home.biopony.de")
        .allow_origin("http://localhost:1234")
        .allow_credentials(true)
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST", "DELETE", "OPTIONS"]);

    let with_tasks_backend = {
        warp::any().map(|| {
            Ok(backend::DefaultTasksBackend::new(
                data::PgFinDb::new(
                    CONNECTION.clone(),
                    backend::TasksBackend::get_logger_context(
                        (*LOGGER).clone(),
                    ),
                ),
                (*LOGGER).clone(),
            ))
        })
    };

    let with_user_backend = {
        warp::any().map(|| {
            Ok(backend::DefaultUserBackend::new(data::PgFinDb::new(
                CONNECTION.clone(),
                backend::UserBackend::get_logger_context((*LOGGER).clone()),
            )))
        })
    };

    // AUTH
    let with_auth = warp::cookie::optional(&auth::SESS_COOKIE_NAME).and_then(
        |opt_sess: Option<String>| match opt_sess {
            Some(sess) => auth::parse_sess(&sess)
                .map_err(|err| warp::reject::custom(FinError::NotLoggedIn)),
            None => Err(warp::reject::custom(FinError::NotLoggedIn)),
        },
    );

    // PROJECTS===============
    // let project_path = warp::path("projects");
    // // GET -> projects
    // let get_projects = warp::get2()
    //     .and(project_path)
    //     .and(warp::path::end())
    //     .and(with_tasks_backend)
    //     .and_then(projects_server::get_all_projects);

    // let project_api = get_projects;

    // TASKS===============
    let task_path = warp::path("tasks");
    // GET -> tasks/incomplete/project/:id
    let get_incomplete_tasks = warp::get2()
        .and(task_path)
        .and(warp::path("incomplete"))
        .and(with_auth)
        .and(warp::path::end())
        .and(with_tasks_backend)
        .and_then(tasks_server::get_incomplete_tasks);
    // POST -> tasks
    let create_task = warp::post2()
        .and(task_path)
        .and(warp::path::end())
        // .and(warp::body::json())
        .and(with_tasks_backend)
        .and_then(tasks_server::create_task);
    let task_api = get_incomplete_tasks.or(create_task);
    // USERS===============
    let user_path = warp::path("users");
    let post_login = warp::post2()
        .and(user_path)
        .and(warp::path("login"))
        .and(warp::body::json())
        .and(with_user_backend)
        .and_then(user_server::login);
    let user_api = post_login;

    // combine apis
    let api = task_api
        // .or(project_api)
        .or(user_api);

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
