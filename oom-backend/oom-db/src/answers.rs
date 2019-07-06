use crate::error::Result;
use crate::model::answers::*;
use diesel::dsl::sql;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::Text;

const PAGE_SIZE: i64 = 10;

pub fn paginate(
    conn: &MysqlConnection,
    post_id: u32,
    page: i64,
    sort: QuerySort,
) -> Result<Vec<Answer>> {
    use crate::schema_additional::answers::dsl::*;

    answers
        .filter(question_post_id.eq(post_id))
        .order((is_best_answer.desc(), sql::<Text>(sort.into()).desc()))
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load::<Answer>(conn)
        .map_err(|e| e.into())
}

pub fn paginate_users(
    conn: &MysqlConnection,
    _user_id: u32,
    page: i64,
    sort: QuerySort,
) -> Result<Vec<Answer>> {
    use crate::schema_additional::answers::dsl::*;

    answers
        .filter(user_id.eq(_user_id))
        .order(sql::<Text>(sort.into()).desc())
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load::<Answer>(conn)
        .map_err(|e| e.into())
}

pub fn by_id(conn: &MysqlConnection, answer_id: u32) -> Result<Option<Answer>> {
    use crate::schema_additional::answers::dsl::*;

    answers
        .filter(id.eq(answer_id))
        .limit(1)
        .load::<Answer>(conn)
        .map_err(|e| e.into())
        .map(|mut a| {
            if a.len() > 0 {
                Some(a.pop().unwrap())
            } else {
                None
            }
        })
}
