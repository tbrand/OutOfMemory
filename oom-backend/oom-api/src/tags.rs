use crate::api_error::api_error;
use crate::auth::{auth, AuthHeader};
use crate::db::list_tags;
use crate::db::tags;
use crate::model::list_tags::*;
use crate::model::tags::*;
use crate::state::State;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn create(
    state: Arc<Mutex<State>>,
    header: AuthHeader,
    new_tag: NewTagBody,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();
    let new_tag = new_tag.as_new_tag(header.user_id);

    tags::create(&conn, new_tag)
        .map_err(|e| {
            error!("error on tags#create={:?}", e);
            api_error("failed to create a tag", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|res| {
            if res == 1 {
                StatusCode::CREATED
            } else {
                StatusCode::BAD_REQUEST
            }
        })
}

pub fn delete(
    tag_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();

    let tag = if let Some(tag) = tags::by_id(&conn, tag_id).map_err(|e| {
        error!("error on tags#by_id={:?}", e);
        api_error(
            "failed to fetch a comment",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })? {
        tag
    } else {
        return Ok(StatusCode::NOT_FOUND);
    };

    if header.user_id != tag.user_id {
        return Ok(StatusCode::BAD_REQUEST);
    }

    tags::delete(&conn, tag_id)
        .map_err(|e| {
            error!("error on tags#delete={:?}", e);
            api_error("failed to delete a tag", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|res| {
            if res == 1 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::BAD_REQUEST
            }
        })
}

pub fn paginate(
    state: Arc<Mutex<State>>,
    query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    list_tags::paginate(&conn, query.as_page(), query.as_sort())
        .map(|t| warp::reply::json(&t))
        .map_err(|e| {
            error!("error on tags#paginate={:?}", e);
            api_error("failed to fetch tags", StatusCode::INTERNAL_SERVER_ERROR)
        })
}

pub fn search(state: Arc<Mutex<State>>, query: Query) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    list_tags::search(&conn, &query.as_q())
        .map(|t| warp::reply::json(&t))
        .map_err(|e| {
            error!("error on tags#search={:?}", e);
            api_error("failed to fetch tags", StatusCode::INTERNAL_SERVER_ERROR)
        })
}

pub fn by_user_id(
    user_id: u32,
    state: Arc<Mutex<State>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    list_tags::by_user_id(&conn, user_id)
        .map(|t| warp::reply::json(&t))
        .map_err(|e| {
            error!("error on tags#paginate_users={:?}", e);
            api_error("failed to fetch tags", StatusCode::INTERNAL_SERVER_ERROR)
        })
}

pub fn create_link_user(
    tag_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    tags::create_link_user(
        &conn,
        LinkUser {
            user_id: header.user_id,
            tag_id,
        },
    )
    .map(|res| {
        if res == 1 {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        }
    })
    .map_err(|e| {
        error!("error on tags#create_link_user={:?}", e);
        api_error(
            "failed to create a tag link on user",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn delete_link_user(
    tag_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    tags::delete_link_user(&conn, header.user_id, tag_id)
        .map(|res| {
            if res == 1 {
                StatusCode::OK
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .map_err(|e| {
            error!("error on tags#delete_link_user={:?}", e);
            api_error(
                "failed to delete a tag link on user",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
}

pub fn create_link_post(
    post_id: u32,
    tag_id: u32,
    state: Arc<Mutex<State>>,
    _header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    tags::create_link_post(&conn, LinkPost { post_id, tag_id })
        .map(|res| {
            if res == 1 {
                StatusCode::OK
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .map_err(|e| {
            error!("error on tags#create_link_post={:?}", e);
            api_error(
                "failed to create a tag link on post",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
}

pub fn delete_link_post(
    post_id: u32,
    tag_id: u32,
    state: Arc<Mutex<State>>,
    _header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    tags::delete_link_post(&conn, post_id, tag_id)
        .map(|res| {
            if res == 1 {
                StatusCode::OK
            } else {
                StatusCode::BAD_REQUEST
            }
        })
        .map_err(|e| {
            error!("error on tags#delete_link_post={:?}", e);
            api_error(
                "failed to delete a tag link on post",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
}

pub fn by_id(tag_id: u32, state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    tags::by_id(&conn, tag_id)
        .map_err(|e| {
            error!("error on tags#by_id={:?}", e);
            api_error("failed to fetch a tag", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .and_then(|t| {
            if let Some(t) = t {
                Ok(warp::reply::json(&t))
            } else {
                Err(api_error("tag not found", StatusCode::NOT_FOUND))
            }
        })
}
