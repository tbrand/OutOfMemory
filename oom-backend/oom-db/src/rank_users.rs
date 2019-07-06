use crate::error::Result;
use crate::model::rank_users::*;
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::Text;

const PAGE_SIZE: i64 = 10;

pub fn paginate(
    conn: &MysqlConnection,
    page: i64,
    sort: QuerySort,
    term: QueryTerm,
) -> Result<Vec<RankUser>> {
    use crate::schema_additional::rank_users::dsl::*;

    let term: (NaiveDateTime, NaiveDateTime) = term.into();

    rank_users
        .filter(created_at.between(term.0, term.1))
        .order(sql::<Text>(sort.into()))
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load(conn)
        .map_err(|e| e.into())
}

pub fn by_user_id(conn: &MysqlConnection, user_id: u32) -> Result<Option<RankUser>> {
    use crate::schema_additional::rank_users::dsl::*;

    rank_users
        .filter(id.eq(user_id))
        .limit(1)
        .load::<RankUser>(conn)
        .map_err(|e| e.into())
        .map(|mut u| {
            if u.len() > 0 {
                Some(u.pop().unwrap())
            } else {
                None
            }
        })
}
