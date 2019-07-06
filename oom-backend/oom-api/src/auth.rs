use crate::api_error::api_error;
use crate::state::State;
use chrono::{Duration, Utc};
use oom_db::users::by_id;
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;
use warp::Filter;

pub const AUTH_HEADER: &'static str = "X-OOM-AUTH";

#[derive(Deserialize, Debug, Clone)]
pub struct AuthHeader {
    pub user_id: u32,
    pub token: String,
}

pub fn auth_header() -> impl Filter<Extract = (AuthHeader,), Error = warp::Rejection> + Copy {
    warp::filters::header::header(AUTH_HEADER).and_then(|s: String| {
        serde_urlencoded::from_str(s.as_str())
            .map_err(|_| api_error("invalid auth header", StatusCode::FORBIDDEN))
    })
}

pub fn auth(state: Arc<Mutex<State>>, header: AuthHeader) -> Result<(), warp::Rejection> {
    let conn = state.lock().unwrap().pool.get().unwrap();

    let mut token_check = false;
    let mut token_expire_check = false;

    match by_id(&conn, header.user_id).map_err(|_| StatusCode::FORBIDDEN) {
        Ok(Some(user)) => {
            if let Some(token) = user.token.as_ref() {
                if token == &header.token {
                    token_check = true;
                }
            }

            if let Some(token_expire) = user.token_expire.as_ref() {
                if *token_expire - Utc::now().naive_utc() > Duration::zero() {
                    token_expire_check = true;
                }
            }

            if token_check && token_expire_check {
                Ok(())
            } else {
                Err(api_error(
                    "user token is not found or expired",
                    StatusCode::FORBIDDEN,
                ))
            }
        }
        Ok(None) => Err(api_error("user not found", StatusCode::FORBIDDEN)),
        Err(e) => {
            log::info!("error on auth={:?}", e);
            Err(api_error(
                "failed to fetch user",
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}
