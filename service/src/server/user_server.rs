use super::auth;
use crate::backend;
use crate::server;
use libpasta;

use http::{self, Response, StatusCode};

pub fn login(
    data: server::LoginForm,
    res_user_backend: Result<impl backend::UserBackend, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // get user with password
    let user_with_pass = res_user_backend?
        .get_user_with_pass(&data.email)
        .map_err(warp::reject::custom)?;

    // verify password
    let opt_user_data = if (libpasta::verify_password(
        &user_with_pass.password,
        &data.password,
    )) {
        Some(server::UserDataApi {
            id: user_with_pass.id,
            email: user_with_pass.email,
        })
    } else {
        None
    };

    match opt_user_data {
        Some(user_data) => auth::resp_with_auth(
            user_data,
            "logged in".to_string(),
            StatusCode::ACCEPTED,
        ),
        None => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("user not found".to_string())
                .unwrap();
            Ok(response)
        }
    }
}

pub fn logout() -> Result<impl warp::Reply, warp::Rejection> {
    let response = Response::builder()
        .status(StatusCode::ACCEPTED)
        .header(
            http::header::SET_COOKIE,
            "sess=;HttpOnly;path=/;expires=Thu, 01 Jan 1970 00:00:00 UTC",
        )
        .body("logged in".to_string())
        .unwrap();
    Ok(response)
}

pub fn signup(
    data: server::LoginForm,
    res_user_backend: Result<impl backend::UserBackend, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let user_backend = res_user_backend?;

    // check email doesnt already exist
    let user_exists = user_backend
        .does_user_exist(&data.email)
        .map_err(warp::reject::custom)?;

    // hash password
    let hash_pass = libpasta::hash_password(&data.password);

    // add user
    if (user_exists) {
        let response = Response::builder()
            .status(StatusCode::CONFLICT)
            .body("user with email already exists".to_string())
            .unwrap();
        Ok(response)
    } else {
        let new_user = user_backend
            .create_user(&data.email, &hash_pass)
            .map_err(warp::reject::custom)?;

        let new_user_api = super::UserDataApi {
            id: new_user.id,
            email: new_user.email,
        };
        auth::resp_with_auth(new_user_api, "".to_string(), StatusCode::CREATED)
    }
}
