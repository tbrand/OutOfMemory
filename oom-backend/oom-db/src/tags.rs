use crate::error::Result;
use crate::model::tags::*;
use crate::schema::tags;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn create(conn: &MysqlConnection, new_tag: NewTag) -> Result<usize> {
    diesel::insert_into(tags::table)
        .values(&new_tag)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn create_link_post(conn: &MysqlConnection, link_post: LinkPost) -> Result<usize> {
    use crate::schema::posts_tags;

    diesel::insert_into(posts_tags::table)
        .values(&link_post)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn delete_link_post(conn: &MysqlConnection, _post_id: u32, _tag_id: u32) -> Result<usize> {
    use crate::schema::posts_tags::dsl::*;

    diesel::delete(posts_tags.filter(post_id.eq(_post_id).and(tag_id.eq(_tag_id))))
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn create_link_user(conn: &MysqlConnection, link_user: LinkUser) -> Result<usize> {
    use crate::schema::users_tags;

    diesel::insert_into(users_tags::table)
        .values(&link_user)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn delete_link_user(conn: &MysqlConnection, _user_id: u32, _tag_id: u32) -> Result<usize> {
    use crate::schema::users_tags::dsl::*;

    diesel::delete(users_tags.filter(user_id.eq(_user_id).and(tag_id.eq(_tag_id))))
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn by_id(conn: &MysqlConnection, _id: u32) -> Result<Option<Tag>> {
    use crate::schema::tags::dsl::*;

    tags.filter(id.eq(_id))
        .order(name)
        .load(conn)
        .map_err(|e| e.into())
        .map(|mut t| {
            if t.len() > 0 {
                Some(t.pop().unwrap())
            } else {
                None
            }
        })
}

pub fn by_post_id(conn: &MysqlConnection, _post_id: u32) -> Result<Vec<Tag>> {
    use crate::schema::posts_tags;
    use crate::schema::posts_tags::dsl::*;
    use crate::schema::tags::dsl::name;

    posts_tags
        .inner_join(tags::table)
        .order(name)
        .filter(posts_tags::post_id.eq(_post_id))
        .select(tags::all_columns)
        .load(conn)
        .map_err(|e| e.into())
}

pub fn by_user_id(conn: &MysqlConnection, _user_id: u32) -> Result<Vec<Tag>> {
    use crate::schema::tags::dsl::name;
    use crate::schema::users_tags;
    use crate::schema::users_tags::dsl::*;

    users_tags
        .inner_join(tags::table)
        .order(name)
        .filter(users_tags::user_id.eq(_user_id))
        .select(tags::all_columns)
        .load(conn)
        .map_err(|e| e.into())
}

pub fn by_name(conn: &MysqlConnection, _name: &str) -> Result<Tag> {
    use crate::schema::tags::dsl::*;

    tags.filter(name.eq(_name))
        .limit(1)
        .load::<Tag>(conn)
        .map_err(|e| e.into())
        .map(|mut t| t.pop().unwrap())
}

pub fn delete(conn: &MysqlConnection, _id: u32) -> Result<usize> {
    use crate::schema::tags::dsl::*;

    diesel::delete(tags.filter(id.eq(_id)))
        .execute(conn)
        .map_err(|e| e.into())
}
