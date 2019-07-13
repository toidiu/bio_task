use crate::backend;
use crate::errors::FinError;
use crate::global::ROOT;
use crate::server;
use std::collections::HashMap;

lazy_static! {
    static ref LOGGER: slog::Logger =
        (*ROOT).clone().new(o!("mod" => "portfolio_server"));
}

pub fn get_all_projects(
    res_backend: Result<impl backend::Backend, warp::Rejection>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let backend = res_backend?;
    let resp = backend.get_all_projects().map_err(|err| {
        lineError!(LOGGER, err);
        warp::reject::custom(FinError::ServerErr)
    })?;

    let reply = serde_json::to_string(&resp).map_err(|err| {
        lineError!(LOGGER, err);
        warp::reject::custom(err)
    })?;
    Ok(reply)
}
