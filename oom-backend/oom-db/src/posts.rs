use crate::error::Result;
use crate::model::posts::*;
use crate::schema::posts;
use diesel::dsl::sql;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::Text;

pub fn create(conn: &MysqlConnection, new_post: NewPost) -> Result<usize> {
    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn update(conn: &MysqlConnection, post_id: u32, update_post: UpdatePost) -> Result<usize> {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.filter(id.eq(post_id)))
        .set(&update_post)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn by_id(conn: &MysqlConnection, _id: u32) -> Result<Option<Post>> {
    use crate::schema::posts::dsl::*;

    posts
        .filter(id.eq(_id))
        .limit(1)
        .load::<Post>(conn)
        .map_err(|e| e.into())
        .map(|mut p| {
            if p.len() > 0 {
                Some(p.pop().unwrap())
            } else {
                None
            }
        })
}

pub fn delete(conn: &MysqlConnection, _id: u32) -> Result<usize> {
    use crate::schema::posts::dsl::*;

    diesel::delete(posts.filter(id.eq(_id)))
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn latest(conn: &MysqlConnection) -> Result<Option<Post>> {
    use crate::schema::posts::dsl::*;

    posts
        .order(sql::<Text>("created_at").desc())
        .limit(1)
        .load::<Post>(conn)
        .map_err(|e| e.into())
        .map(|mut p| {
            if p.len() > 0 {
                Some(p.pop().unwrap())
            } else {
                None
            }
        })
}
