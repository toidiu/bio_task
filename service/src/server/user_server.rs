use super::auth;
use crate::backend;
use crate::global::CONFIG;
use crate::server;
use libpasta;

use http::{self, Response, StatusCode};

pub fn login(
    data: server::LoginForm,
    res_user_backend: Result<impl backend::UserBackend, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // see if the pass matches the one in file
    println!("{}", data.password);
    println!("{:?}", *CONFIG);
    if (data.password == *CONFIG.password) {
        auth::resp_with_auth(
            data.password,
            "logged in".to_string(),
            StatusCode::ACCEPTED,
        )
    } else {
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body("bad password".to_string())
            .unwrap();
        Ok(response)
    }

    // match opt_user_data {
    //     Some(user_data) => auth::resp_with_auth(
    //         user_data,
    //         "logged in".to_string(),
    //         StatusCode::ACCEPTED,
    //     ),
    //     None => {
    //         let response = Response::builder()
    //             .status(StatusCode::NOT_FOUND)
    //             .body("user not found".to_string())
    //             .unwrap();
    //         Ok(response)
    //     }
    // }
    // Ok("asdf")
}
