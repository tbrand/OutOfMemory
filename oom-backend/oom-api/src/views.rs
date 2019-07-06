use crate::api_error::api_error;
use crate::auth::{auth, AuthHeader};
use crate::db::views;
use crate::model::views::*;
use crate::state::State;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

pub fn upsert(
    post_id: u32,
    state: Arc<Mutex<State>>,
    header: AuthHeader,
) -> Result<impl warp::Reply, warp::Rejection> {
    auth(state.clone(), header.clone())?;

    let conn = state.lock().unwrap().pool.get().unwrap();
    let new_view = NewView {
        user_id: header.user_id,
        post_id,
    };

    views::upsert(&conn, new_view)
        .map_err(|e| {
            error!("error on views#upsert={:?}", e);
            api_error("failed to upsert a view", StatusCode::INTERNAL_SERVER_ERROR)
        })
        .map(|_| StatusCode::CREATED)
}
