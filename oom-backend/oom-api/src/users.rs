use crate::api_error::api_error;
use crate::db::rank_users;
use crate::model::rank_users::*;
use crate::state::State;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn paginate(
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    rank_users::paginate(&conn, query.as_page(), query.as_sort(), query.as_term())
        .map(|u| warp::reply::json(&u))
        .map_err(|e| {
            error!("error on users#paginate={:?}", e);
            api_error("failed to fetch users", StatusCode::INTERNAL_SERVER_ERROR)
        })
}

pub fn by_id(user_id: u32, state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    rank_users::by_user_id(&conn, user_id)
        .map(|u| warp::reply::json(&u))
        .map_err(|e| {
            error!("error on users#by_user_id={:?}", e);
            api_error("failed to fetch user", StatusCode::INTERNAL_SERVER_ERROR)
        })
}
