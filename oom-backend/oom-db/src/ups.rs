use crate::error::Result;
use crate::model::ups::*;
use crate::schema::ups;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn upsert(conn: &MysqlConnection, new_up: NewUp) -> Result<usize> {
    diesel::replace_into(ups::table)
        .values(&new_up)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn is_up(conn: &MysqlConnection, _user_id: u32, _post_id: u32) -> Result<Option<bool>> {
    use crate::schema::ups::dsl::*;

    ups.filter(post_id.eq(_post_id))
        .filter(user_id.eq(_user_id))
        .limit(1)
        .load(conn)
        .map_err(|e| e.into())
        .map(|up: Vec<Up>| {
            if up.len() > 0 {
                Some(up[0].is_up)
            } else {
                None
            }
        })
}
