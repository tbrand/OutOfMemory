use crate::api_error::api_error;
use crate::auth::{auth, AuthHeader};
use crate::db::comments;
use crate::model::comments::*;
use crate::state::State;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn create(
    post_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
    new_comment: NewCommentBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();
    let new_comment = new_comment.as_new_comment(header.user_id, post_id);

    comments::create(&conn, new_comment)
        .map_err(|e| {
            error!("error on comments#create={:?}", e);
            api_error(
                "failed to create a comment",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .map(|res| {
            if res == 1 {
                StatusCode::CREATED
            } else {
                StatusCode::BAD_REQUEST
            }
        })
}

pub fn by_post_id(
    post_id: u32,
    state: Arc<Mutex<State>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    comments::by_post_id(&conn, post_id)
        .map(|comments| warp::reply::json(&comments))
        .map_err(|e| {
            error!("error on comments#by_post_id={:?}", e);
            api_error(
                "failed to fetch comments",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
}

pub fn delete(
    comment_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();

    let comment = if let Some(comment) = comments::by_id(&conn, comment_id).map_err(|e| {
        error!("error on comments#by_id={:?}", e);
        api_error(
            "failed to fetch a comment",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })? {
        comment
    } else {
        return Ok(StatusCode::NOT_FOUND);
    };

    if header.user_id != comment.user_id {
        return Ok(StatusCode::BAD_REQUEST);
    }

    comments::delete(&conn, comment_id)
        .map_err(|e| {
            error!("error on comments#delete={:?}", e);
            api_error(
                "failed to delete a comment",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .map(|res| {
            if res == 1 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::BAD_REQUEST
            }
        })
}
