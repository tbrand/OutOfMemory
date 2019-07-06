use crate::api_error::api_error;
use crate::db::questions;
use crate::model::questions::*;
use crate::state::State;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn paginate(
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    questions::paginate(
        &conn,
        query.as_page(),
        query.as_flag(),
        query.as_sort(),
        query.as_term(),
    )
    .map(|q| warp::reply::json(&q))
    .map_err(|e| {
        error!("error on questions#paginate={:?}", e);
        api_error(
            "failed to fetch questions",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn paginate_users(
    user_id: u32,
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    questions::paginate_users(
        &conn,
        user_id,
        query.as_page(),
        query.as_flag(),
        query.as_sort(),
        query.as_term(),
    )
    .map(|q| warp::reply::json(&q))
    .map_err(|e| {
        error!("error on questions#paginate_users={:?}", e);
        api_error(
            "failed to fetch questions",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn paginate_users_answers(
    user_id: u32,
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    questions::paginate_users_answers(
        &conn,
        user_id,
        query.as_page(),
        query.as_sort(),
        query.as_term(),
    )
    .map(|q| warp::reply::json(&q))
    .map_err(|e| {
        error!("error on questions#paginate_users_answers={:?}", e);
        api_error(
            "failed to fetch questions",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn paginate_tags(
    tag_id: u32,
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    questions::paginate_tags(
        &conn,
        tag_id,
        query.as_page(),
        query.as_flag(),
        query.as_sort(),
        query.as_term(),
    )
    .map(|q| warp::reply::json(&q))
    .map_err(|e| {
        error!("error on questions#paginate_tags={:?}", e);
        api_error(
            "failed to fetch questions",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn search(state: Arc<Mutex<State>>, query: Query) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    questions::search(
        &conn,
        query.as_page(),
        query.as_flag(),
        query.as_sort(),
        query.as_term(),
        &query.as_q(),
    )
    .map(|q| warp::reply::json(&q))
    .map_err(|e| {
        error!("error on questions#search={:?}", e);
        api_error(
            "failed to fetch questions",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn by_id(
    question_id: u32,
    state: Arc<Mutex<State>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    questions::by_id(&conn, question_id)
        .map_err(|e| {
            error!("error on questions#by_id={:?}", e);
            api_error(
                "failed to fetch a question",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .and_then(|q| {
            if let Some(q) = q {
                Ok(warp::reply::json(&q))
            } else {
                Err(api_error("question not found", StatusCode::NOT_FOUND))
            }
        })
}
