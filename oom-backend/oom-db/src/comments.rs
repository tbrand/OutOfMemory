use crate::error::Result;
use crate::model::comments::*;
use crate::schema::comments;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn create(conn: &MysqlConnection, new_comment: NewComment) -> Result<usize> {
    diesel::insert_into(comments::table)
        .values(&new_comment)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn by_id(conn: &MysqlConnection, _id: u32) -> Result<Option<Comment>> {
    use crate::schema::comments::dsl::*;

    comments
        .filter(id.eq(_id))
        .load(conn)
        .map_err(|e| e.into())
        .map(|mut c| {
            if c.len() > 0 {
                Some(c.pop().unwrap())
            } else {
                None
            }
        })
}

pub fn by_post_id(conn: &MysqlConnection, _post_id: u32) -> Result<Vec<Comment>> {
    use crate::schema::comments::dsl::*;

    comments
        .filter(post_id.eq(_post_id))
        .load(conn)
        .map_err(|e| e.into())
}

pub fn delete(conn: &MysqlConnection, _id: u32) -> Result<usize> {
    use crate::schema::comments::dsl::*;

    diesel::delete(comments.filter(id.eq(_id)))
        .execute(conn)
        .map_err(|e| e.into())
}
