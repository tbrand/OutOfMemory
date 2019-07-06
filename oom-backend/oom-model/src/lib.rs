#![deny(warnings)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde;

pub mod answers;
pub mod comments;
pub mod list_tags;
pub mod posts;
pub mod questions;
pub mod rank_users;
pub mod tags;
pub mod term_builder;
pub mod ups;
pub mod users;
pub mod users_analysis;
pub mod views;

pub use oom_schema::schema;
pub use oom_schema::schema_additional;
