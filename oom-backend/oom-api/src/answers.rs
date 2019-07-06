use crate::api_error::api_error;
use crate::db::answers;
use crate::model::answers::*;
use crate::state::State;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn paginate(
    post_id: u32,
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    answers::paginate(&conn, post_id, query.as_page(), query.as_sort())
        .map(|a| warp::reply::json(&a))
        .map_err(|e| {
            error!("error on answers#paginate={:?}", e);
            api_error("failed to fetch answers", StatusCode::INTERNAL_SERVER_ERROR)
        })
}

pub fn paginate_users(
    user_id: u32,
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    answers::paginate_users(&conn, user_id, query.as_page(), query.as_sort())
        .map(|a| warp::reply::json(&a))
        .map_err(|e| {
            error!("error on answers#paginate_users={:?}", e);
            api_error("failed to fetch answers", StatusCode::INTERNAL_SERVER_ERROR)
        })
}

pub fn by_id(
    answer_id: u32,
    state: Arc<Mutex<State>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    answers::by_id(&conn, answer_id)
        .map_err(|e| {
            error!("error on answers#by_id={:?}", e);
            api_error(
                "failed to fetch an answer",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .and_then(|a| {
            if let Some(a) = a {
                Ok(warp::reply::json(&a))
            } else {
                Err(api_error("answer not found", StatusCode::NOT_FOUND))
            }
        })
}
