use crate::backend;
use crate::data;
use crate::errors::{ErrMessage, FinError};
use crate::global::{CONFIG, ROOT};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use http::StatusCode;

use warp::{Filter, Rejection};

mod api;
mod auth;
mod tasks_server;

pub use api::*;
pub use auth::UserId;

lazy_static! {
    static ref CONNECTION: r2d2::Pool<PostgresConnectionManager> = {
        let manager = PostgresConnectionManager::new(
            CONFIG.database.url.to_string(),
            TlsMode::None,
        )
        .unwrap();
        r2d2::Pool::builder()
            .max_size(CONFIG.database.pool_size)
            .build(manager)
            .expect("Failed to create pool")
    };
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "server"));
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

    let with_task_backend = {
        warp::any().map(|| match CONNECTION.get() {
            Ok(conn) => Ok(backend::DefaultTasksBackend::new(
                data::PgFinDb::new(
                    // conn,
                    backend::TasksBackend::get_logger_context(
                        (*LOGGER).clone(),
                    ),
                ),
                (*LOGGER).clone(),
            )),
            Err(err) => {
                error!(LOGGER, "{}: {}", line!(), err);
                Err(warp::reject::custom(FinError::DatabaseErr))
            }
        })
    };

    // TASKS===============
    let task_path = warp::path("tasks");
    // GET -> tasks/incomplete
    //     get incomplete (sort by date)
    let get_incomplete_task = warp::get2()
        .and(task_path)
        .and(warp::path("incomplete"))
        .and(warp::path::end())
        .and(with_task_backend)
        .and_then(tasks_server::get_tasks);

    // GET -> tasks/completed?page=0
    //     get completed (paginated)
    // // GET -> tasks/actual/buy/?goal_id=1&amount=1
    // let get_buy_next = warp::get2()
    //     .and(task_path)
    //     .and(warp::path("actual"))
    //     .and(warp::path("buy"))
    //     .and(with_auth)
    //     .and(warp::query())
    //     .and(with_task_backend)
    //     .and_then(tasks_server::get_buy_next);

    // GET -> tasks/incomplete/projects/:id
    //     get by project (incomplete and sort by date)
    // let get_port_a_by_id = warp::get2()
    //     .and(task_path)
    //     .and(warp::path("actual"))
    //     .and(warp::path::param2::<i64>())
    //     .and(warp::path::end())
    //     .and(with_auth)
    //     .and(with_task_backend)
    //     .and_then(tasks_server::get_portfolio_a);

    // GET -> tasks/incomplete/projects
    //     get by all projects (incomplete and sort by date)
    // let get_port_a_list = warp::get2()
    //     .and(task_path)
    //     .and(warp::path("actual"))
    //     .and(warp::path::end())
    //     .and(with_auth)
    //     .and(with_task_backend)
    //     .and_then(tasks_server::get_port_a_list);

    // POST -> tasks
    let create_task = warp::post2()
        .and(task_path)
        .and(warp::path::end())
        // .and(warp::body::json())
        .and(with_task_backend)
        .and_then(tasks_server::create_task);

    let task_api = get_incomplete_task.or(create_task);

    // DEPENDENCY===============
    //     post create dependency
    //     get dependency by task id

    // combine apis
    let api = task_api;

    let routes = api.recover(recover_error).with(with_cors);
    warp::serve(routes).run(([127, 0, 0, 1], CONFIG.app.port));
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
