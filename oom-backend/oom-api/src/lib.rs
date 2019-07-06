#![deny(warnings)]
#![recursion_limit = "128"]

#[macro_use]
extern crate warp;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

pub mod answers;
pub mod api_error;
pub mod auth;
pub mod comments;
pub mod config;
pub mod cors;
pub mod login;
pub mod posts;
pub mod questions;
pub mod state;
pub mod tags;
pub mod ups;
pub mod users;
pub mod views;

use crate::auth::auth_header;
use crate::cors::cors;
use crate::state::State;
pub use oom_db as db;
pub use oom_model as model;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use warp::Filter;

pub const BODY_SIZE_LIMIT: u64 = 1024 * 1024 * 256;

pub fn listen(
    addr: impl Into<SocketAddr> + 'static,
    database_url: impl Into<String>,
    config_path: impl Into<String>,
) {
    let state = Arc::new(Mutex::new(State::new(database_url, config_path)));
    let state = warp::any().map(move || state.clone());

    let root = warp::path::end().and(warp::get2()).map(|| "OK");

    let posts_create = path!("posts")
        .and(warp::path::end())
        .and(warp::post2())
        .and(state.clone())
        .and(auth_header())
        .and(warp::body::content_length_limit(BODY_SIZE_LIMIT))
        .and(warp::body::json())
        .and_then(posts::create);

    let posts_update_content = path!("posts" / u32)
        .and(warp::path::end())
        .and(warp::patch())
        .and(state.clone())
        .and(auth_header())
        .and(warp::body::content_length_limit(BODY_SIZE_LIMIT))
        .and(warp::body::json())
        .and_then(posts::update_content);

    let posts_update_best_answer = path!("posts" / u32 / "best")
        .and(warp::path::end())
        .and(warp::patch())
        .and(state.clone())
        .and(auth_header())
        .and(warp::body::content_length_limit(BODY_SIZE_LIMIT))
        .and(warp::body::json())
        .and_then(posts::update_best_answer);

    let posts_delete = path!("posts" / u32)
        .and(warp::path::end())
        .and(warp::delete2())
        .and(state.clone())
        .and(auth_header())
        .and_then(posts::delete);

    let posts_comments_create = path!("posts" / u32 / "comments")
        .and(warp::path::end())
        .and(warp::post2())
        .and(state.clone())
        .and(auth_header())
        .and(warp::body::content_length_limit(BODY_SIZE_LIMIT))
        .and(warp::body::json())
        .and_then(comments::create);

    let posts_comments_by_post_id = path!("posts" / u32 / "comments")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(comments::by_post_id);

    let posts_tags = path!("posts" / u32 / "tags")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(posts::tags);

    let posts_tags_create = path!("posts" / u32 / "tags" / u32)
        .and(warp::path::end())
        .and(warp::post2())
        .and(state.clone())
        .and(auth_header())
        .and_then(tags::create_link_post);

    let posts_tags_delete = path!("posts" / u32 / "tags" / u32)
        .and(warp::path::end())
        .and(warp::delete2())
        .and(state.clone())
        .and(auth_header())
        .and_then(tags::delete_link_post);

    let posts_ups_upsert = path!("posts" / u32 / "ups")
        .and(warp::path::end())
        .and(warp::patch())
        .and(state.clone())
        .and(auth_header())
        .and(warp::body::content_length_limit(BODY_SIZE_LIMIT))
        .and(warp::body::json())
        .and_then(ups::upsert);

    let posts_ups_is_up = path!("posts" / u32 / "ups")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(auth_header())
        .and_then(ups::is_up);

    let posts_views_upsert = path!("posts" / u32 / "views")
        .and(warp::path::end())
        .and(warp::patch())
        .and(state.clone())
        .and(auth_header())
        .and_then(views::upsert);

    let questions_paginate = path!("questions")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(questions::paginate);

    let questions_tag_paginate = path!("questions" / "tag" / u32)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(questions::paginate_tags);

    let questions_search = path!("questions" / "search")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(questions::search);

    let questions_by_question_id = path!("questions" / u32)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(questions::by_id);

    let questions_answers = path!("questions" / u32 / "answers")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(answers::paginate);

    let answers_by_answer_id = path!("answers" / u32)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(answers::by_id);

    let users_login = path!("users" / "login")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(login::login);

    let users_login_info = path!("users" / "login" / "info")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(login::login_info);

    let users_paginate = path!("users")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(users::paginate);

    let users_paginate_questions = path!("users" / u32 / "questions")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(questions::paginate_users);

    let users_paginate_answers = path!("users" / u32 / "answers")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(answers::paginate_users);

    let users_paginate_answers_questions = path!("users" / u32 / "answers" / "questions")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(questions::paginate_users_answers);

    let users_paginate_tags = path!("users" / u32 / "tags")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(tags::by_user_id);

    let users_tags_create = path!("users" / "tags" / u32)
        .and(warp::path::end())
        .and(warp::post2())
        .and(state.clone())
        .and(auth_header())
        .and_then(tags::create_link_user);

    let users_tags_delete = path!("users" / "tags" / u32)
        .and(warp::path::end())
        .and(warp::delete2())
        .and(state.clone())
        .and(auth_header())
        .and_then(tags::delete_link_user);

    let users_by_user_id = path!("users" / u32)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(users::by_id);

    let tags_by_tag_id = path!("tags" / u32)
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and_then(tags::by_id);

    let tags_create = path!("tags")
        .and(warp::path::end())
        .and(warp::post2())
        .and(state.clone())
        .and(auth_header())
        .and(warp::body::content_length_limit(BODY_SIZE_LIMIT))
        .and(warp::body::json())
        .and_then(tags::create);

    let tags_delete = path!("tags" / u32)
        .and(warp::path::end())
        .and(warp::delete2())
        .and(state.clone())
        .and(auth_header())
        .and_then(tags::delete);

    let tags_paginate = path!("tags")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(tags::paginate);

    let tags_search = path!("tags" / "search")
        .and(warp::path::end())
        .and(warp::get2())
        .and(state.clone())
        .and(warp::query())
        .and_then(tags::search);

    let comments_delete = path!("comments" / u32)
        .and(warp::path::end())
        .and(warp::delete2())
        .and(state.clone())
        .and(auth_header())
        .and_then(comments::delete);

    let cors = warp::any().and(warp::options()).and_then(cors);

    let routes = root
        .or(posts_create)
        .or(posts_update_content)
        .or(posts_update_best_answer)
        .or(posts_delete)
        .or(posts_comments_create)
        .or(posts_comments_by_post_id)
        .or(posts_tags)
        .or(posts_tags_create)
        .or(posts_tags_delete)
        .or(posts_ups_upsert)
        .or(posts_ups_is_up)
        .or(posts_views_upsert)
        .or(questions_paginate)
        .or(questions_tag_paginate)
        .or(questions_search)
        .or(questions_by_question_id)
        .or(questions_answers)
        .or(answers_by_answer_id)
        .or(users_login)
        .or(users_login_info)
        .or(users_paginate)
        .or(users_paginate_questions)
        .or(users_paginate_answers)
        .or(users_paginate_answers_questions)
        .or(users_paginate_tags)
        .or(users_tags_create)
        .or(users_tags_delete)
        .or(users_by_user_id)
        .or(tags_by_tag_id)
        .or(tags_create)
        .or(tags_delete)
        .or(tags_paginate)
        .or(tags_search)
        .or(comments_delete)
        .or(cors)
        .recover(api_error::as_reply)
        .with(warp::log("oom"))
        .with(
            warp::cors()
                .allow_any_origin()
                .allow_methods(vec!["GET", "POST", "PATCH", "DELETE", "OPTIONS"])
                .allow_headers(vec!["X-OOM-AUTH", "Content-Type"]),
        );

    warp::serve(routes).run(addr);
}
