use crate::api_error::{api_error, ApiError};
use crate::auth::{auth, AuthHeader};
use crate::db::posts;
use crate::model::posts::*;
use crate::state::State;
use diesel::connection::Connection;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn create(
    state: Arc<Mutex<State>>,
    header: AuthHeader,
    new_post: NewPostBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();
    let new_post = new_post.as_new_post(header.user_id);

    conn.transaction(|| {
        posts::create(&conn, new_post)
            .map_err(|e| {
                error!("error on posts#create={:?}", e);
                ApiError::new(
                    "failed to create a post".to_owned(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })
            .and_then(|res| {
                if res == 1 {
                    posts::latest(&conn)
                        .map(|post| {
                            let res = NewPostResponse {
                                id: post.unwrap().id,
                            };

                            Ok(warp::reply::json(&res))
                        })
                        .map_err(|e| {
                            log::error!("error on posts#create={:?}", e);
                            ApiError::new(
                                "failed to create a post".to_owned(),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            )
                        })
                } else {
                    Err(ApiError::new(
                        "failed to create a post".to_owned(),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            })
    })
    .map_err(warp::reject::custom)
}

pub fn update_content(
    post_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
    update_content: UpdateContent,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();

    let post = if let Some(post) = posts::by_id(&conn, post_id).map_err(|e| {
        error!("error on posts#by_id={:?}", e);
        api_error("failed to fetch a post", StatusCode::INTERNAL_SERVER_ERROR)
    })? {
        post
    } else {
        return Ok(StatusCode::NOT_FOUND);
    };

    if header.user_id != post.user_id {
        return Ok(StatusCode::BAD_REQUEST);
    }

    let update_post = UpdatePost {
        title: update_content.title,
        body: update_content.body,
        is_best_answer: None,
    };

    posts::update(&conn, post_id, update_post)
        .map_err(|e| {
            error!("error on posts#update={:?}", e);
            api_error("failed to update a post", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|res| {
            if res == 1 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::BAD_REQUEST
            }
        })
}

pub fn update_best_answer(
    post_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
    update_best_answer: UpdateBestAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();

    // TODO
    // Check the submitter is same as question's submitter.
    let _post = if let Some(post) = posts::by_id(&conn, post_id).map_err(|e| {
        error!("error on posts#by_id={:?}", e);
        api_error("failed to fetch a post", StatusCode::INTERNAL_SERVER_ERROR)
    })? {
        post
    } else {
        return Ok(StatusCode::NOT_FOUND);
    };

    let update_post = UpdatePost {
        title: None,
        body: None,
        is_best_answer: Some(update_best_answer.is_best_answer),
    };

    posts::update(&conn, post_id, update_post)
        .map_err(|e| {
            error!("error on posts#update={:?}", e);
            api_error("failed to update a post", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|res| {
            if res == 1 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::BAD_REQUEST
            }
        })
}

pub fn delete(
    post_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();

    let post = if let Some(post) = posts::by_id(&conn, post_id).map_err(|e| {
        error!("error on posts#by_id={:?}", e);
        api_error("failed to fetch a post", StatusCode::INTERNAL_SERVER_ERROR)
    })? {
        post
    } else {
        return Ok(StatusCode::NOT_FOUND);
    };

    if header.user_id != post.user_id {
        return Ok(StatusCode::BAD_REQUEST);
    }

    posts::delete(&conn, post_id)
        .map_err(|e| {
            error!("error on posts#delete={:?}", e);
            api_error("failed to delete a post", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|res| {
            if res == 1 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::BAD_REQUEST
            }
        })
}

pub fn tags(post_id: u32, state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    use crate::db::tags::by_post_id;

    let conn = state.lock().unwrap().pool.get().unwrap();

    by_post_id(&conn, post_id)
        .map(|t| warp::reply::json(&t))
        .map_err(|e| {
            error!("error on posts#tags={:?}", e);
            api_error("failed to fetch tags", StatusCode::INTERNAL_SERVER_ERROR)
        })
}
