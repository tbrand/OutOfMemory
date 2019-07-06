use crate::api_error::api_error;
use crate::db::users::*;
use crate::model::users::*;
use crate::state::State;
use chrono::Utc;
use diesel::mysql::MysqlConnection;
use futures::Future;
use rand::Rng;
use reqwest::r#async::Client;
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use warp::http::StatusCode;

#[derive(Deserialize, Debug)]
pub struct Query {
    code: String,
}

#[derive(Serialize, Debug)]
pub struct OAuth {
    client_id: String,
    client_secret: String,
    code: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    user_id: u32,
    token: String,
}

#[derive(Serialize, Debug)]
pub struct LoginInfoResponse {
    client_id: String,
    oauth_authorize: String,
}

pub fn login_info(state: Arc<Mutex<State>>) -> Result<impl warp::Reply, warp::Rejection> {
    let state = state.lock().unwrap();
    let client_id = state.config.github.client_id.clone();
    let oauth_authorize = state.config.github.oauth_authorize.clone();

    Ok(warp::reply::json(&LoginInfoResponse {
        client_id,
        oauth_authorize,
    }))
}

pub fn login(
    state: Arc<Mutex<State>>,
    query: Query,
) -> Box<Future<Item = impl warp::Reply, Error = warp::Rejection> + Send> {
    let state = state.lock().unwrap();
    let conn = state.pool.get().unwrap();
    let api_endpoint = state.config.github.api_endpoint.clone();
    let client_id = state.config.github.client_id.clone();
    let client_secret = state.config.github.client_secret.clone();
    let oauth_access_token = state.config.github.oauth_access_token.clone();

    let oauth = OAuth {
        client_id,
        client_secret,
        code: query.code,
    };

    let f = Client::new()
        .post(&oauth_access_token)
        .header(reqwest::header::ACCEPT, "application/json")
        .json(&oauth)
        .send()
        .and_then(|mut res| res.json::<serde_json::Value>())
        .map_err(|e| {
            log::error!("error on login={:?}", e);
            api_error(
                "failed to get access token from github",
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        })
        .and_then(|json| {
            json.get("access_token")
                .map(|access_token| access_token.as_str().unwrap().to_owned())
                .ok_or_else(|| {
                    api_error(
                        "failed to get access token from github",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })
        })
        .and_then(move |access_token| {
            fetch_user(&api_endpoint, &access_token)
                .map_err(|e| {
                    log::error!("error on login={:?}", e);
                    api_error(
                        "failed to fetch user info from github",
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                })
                .and_then(move |fetch_user| create_or_update(&conn, &access_token, fetch_user))
        });

    Box::new(f)
}

#[derive(Debug)]
struct FetchUser {
    name: String,
    avatar_url: String,
}

fn fetch_user(
    api_endpoint: &str,
    access_token: &str,
) -> impl Future<Item = FetchUser, Error = reqwest::Error> {
    Client::new()
        .get(&format!("{}/user", api_endpoint))
        .bearer_auth(access_token)
        .send()
        .and_then(|mut res| {
            res.json::<serde_json::Value>().map(|user| FetchUser {
                name: user["login"].as_str().unwrap().to_owned(),
                avatar_url: user["avatar_url"].as_str().unwrap().to_owned(),
            })
        })
}

fn create_or_update(
    conn: &MysqlConnection,
    access_token: &str,
    fetch_user: FetchUser,
) -> Result<impl warp::Reply, warp::Rejection> {
    let random_bytes = rand::thread_rng().gen::<[u8; 32]>();
    let mut hasher = Sha256::new();

    hasher.input(random_bytes);

    let token: String = hasher
        .result()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("");

    let res = if let Some(user) = by_name(&conn, &fetch_user.name).map_err(|e| {
        log::error!("error on login={:?}", e);
        api_error(
            "failed to get user by name",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })? {
        update(
            &conn,
            user.id,
            UpdateUser {
                name: None,
                avatar_url: None,
                token: Some(Some(token.clone())),
                token_expire: Some(Some(Utc::now().naive_utc() + chrono::Duration::days(1))),
                access_token: Some(Some(access_token.to_owned())),
            },
        )
        .map_err(|e| {
            log::error!("error on login={:?}", e);
            api_error("failed to update user", StatusCode::INTERNAL_SERVER_ERROR)
        })?
    } else {
        create(
            &conn,
            NewUser {
                name: fetch_user.name.clone(),
                avatar_url: fetch_user.avatar_url,
                token: Some(token.clone()),
                token_expire: Some(Utc::now().naive_utc() + chrono::Duration::days(1)),
                access_token: Some(access_token.to_owned()),
            },
        )
        .map_err(|e| {
            log::error!("error on login={:?}", e);
            api_error("failed to create user", StatusCode::INTERNAL_SERVER_ERROR)
        })?
    };

    if res == 1 {
        let user = by_name(&conn, &fetch_user.name)
            .map_err(|e| {
                log::error!("error on login={:?}", e);
                api_error("failed to get user", StatusCode::INTERNAL_SERVER_ERROR)
            })?
            .unwrap();

        Ok(warp::reply::json(&LoginResponse {
            user_id: user.id,
            token,
        }))
    } else {
        log::warn!("failed to create or update user");
        Err(api_error(
            "failed to login",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
