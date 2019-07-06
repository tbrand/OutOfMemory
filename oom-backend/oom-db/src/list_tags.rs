use crate::error::Result;
use crate::model::list_tags::*;
use diesel::dsl::sql;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::Text;

const PAGE_SIZE: i64 = 30;

pub fn paginate(conn: &MysqlConnection, page: i64, sort: QuerySort) -> Result<Vec<ListTag>> {
    use crate::schema_additional::list_tags::dsl::*;

    list_tags
        .order(sql::<Text>(sort.into()))
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load(conn)
        .map_err(|e| e.into())
}

pub fn paginate_users(
    conn: &MysqlConnection,
    _user_id: u32,
    page: i64,
    sort: QuerySort,
) -> Result<Vec<ListTag>> {
    use crate::schema_additional::list_tags::dsl::*;

    list_tags
        .filter(user_id.eq(_user_id))
        .order(sql::<Text>(sort.into()))
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load(conn)
        .map_err(|e| e.into())
}

pub fn by_user_id(conn: &MysqlConnection, _user_id: u32) -> Result<Vec<ListTag>> {
    use crate::schema::users_tags;
    use crate::schema::users_tags::dsl::*;
    use crate::schema_additional::list_tags;
    use crate::schema_additional::list_tags::dsl::name;

    users_tags
        .inner_join(list_tags::table)
        .order(name)
        .filter(users_tags::user_id.eq(_user_id))
        .select(list_tags::all_columns)
        .load(conn)
        .map_err(|e| e.into())
}

pub fn search(conn: &MysqlConnection, q: &str) -> Result<Vec<ListTag>> {
    use crate::schema_additional::list_tags::dsl::*;

    list_tags
        .filter(name.like(format!("{}%", q)))
        .order(name)
        .offset(0)
        .limit(PAGE_SIZE)
        .load(conn)
        .map_err(|e| e.into())
}
