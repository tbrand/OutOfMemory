use crate::error::Result;
use crate::model::views::*;
use crate::schema::views;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn upsert(conn: &MysqlConnection, new_view: NewView) -> Result<usize> {
    diesel::replace_into(views::table)
        .values(&new_view)
        .execute(conn)
        .map_err(|e| e.into())
}
