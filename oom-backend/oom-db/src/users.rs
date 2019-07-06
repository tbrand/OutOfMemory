use crate::error::Result;
use crate::model::users::*;
use crate::schema::users;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn create(conn: &MysqlConnection, new_user: NewUser) -> Result<usize> {
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn update(conn: &MysqlConnection, user_id: u32, update_user: UpdateUser) -> Result<usize> {
    use crate::schema::users::dsl::*;

    diesel::update(users.filter(id.eq(user_id)))
        .set(&update_user)
        .execute(conn)
        .map_err(|e| e.into())
}

pub fn by_id(conn: &MysqlConnection, _id: u32) -> Result<Option<User>> {
    use crate::schema::users::dsl::*;

    users
        .filter(id.eq(_id))
        .limit(1)
        .load(conn)
        .map_err(|e| e.into())
        .map(|mut u| {
            if u.len() > 0 {
                Some(u.pop().unwrap())
            } else {
                None
            }
        })
}

pub fn by_name(conn: &MysqlConnection, _name: &str) -> Result<Option<User>> {
    use crate::schema::users::dsl::*;

    users
        .filter(name.eq(_name))
        .limit(1)
        .load(conn)
        .map_err(|e| e.into())
        .map(|mut u| {
            if u.len() > 0 {
                Some(u.pop().unwrap())
            } else {
                None
            }
        })
}
