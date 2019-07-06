use crate::api_error::api_error;
use crate::auth::{auth, AuthHeader};
use crate::db::ups;
use crate::model::ups::*;
use crate::state::State;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn upsert(
    post_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
    new_up: NewUpBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();
    let new_up = new_up.as_new_up(header.user_id, post_id);

    ups::upsert(&conn, new_up)
        .map_err(|e| {
            error!("error on ups#upsert={:?}", e);
            api_error("failed to upsert an up", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|_| StatusCode::CREATED)
}

pub fn is_up(
    post_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();

    ups::is_up(&conn, header.user_id, post_id)
        .map_err(|e| {
            error!("error on ups#is_up={:?}", e);
            api_error("failed to get ups", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|u| {
            let is_up = IsUp { is_up: u };
            warp::reply::json(&is_up)
        })
}
