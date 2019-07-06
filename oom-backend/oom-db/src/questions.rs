use crate::error::Result;
use crate::model::questions::*;
use chrono::NaiveDateTime;
use diesel::dsl::sql;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::{Bool, Text};

const PAGE_SIZE: i64 = 30;

pub fn paginate(
    conn: &MysqlConnection,
    page: i64,
    flag: QueryFlag,
    sort: QuerySort,
    term: QueryTerm,
) -> Result<Vec<Question>> {
    use crate::schema_additional::questions::dsl::*;

    let term: (NaiveDateTime, NaiveDateTime) = term.into();

    questions
        .filter(sql::<Bool>(flag.into()).and(created_at.between(term.0, term.1)))
        .order(sql::<Text>(sort.into()).desc())
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load::<Question>(conn)
        .map_err(|e| e.into())
}

pub fn paginate_tags(
    conn: &MysqlConnection,
    _tag_id: u32,
    page: i64,
    flag: QueryFlag,
    sort: QuerySort,
    term: QueryTerm,
) -> Result<Vec<Question>> {
    use crate::schema::posts_tags;
    use crate::schema_additional::questions::all_columns;
    use crate::schema_additional::questions::dsl::*;

    let term: (NaiveDateTime, NaiveDateTime) = term.into();

    questions
        .inner_join(posts_tags::table)
        .select(all_columns)
        .filter(
            sql::<Bool>(flag.into())
                .and(created_at.between(term.0, term.1))
                .and(posts_tags::dsl::tag_id.eq(_tag_id)),
        )
        .order(sql::<Text>(sort.into()).desc())
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load::<Question>(conn)
        .map_err(|e| e.into())
}

pub fn paginate_users(
    conn: &MysqlConnection,
    _user_id: u32,
    page: i64,
    flag: QueryFlag,
    sort: QuerySort,
    term: QueryTerm,
) -> Result<Vec<Question>> {
    use crate::schema_additional::questions::dsl::*;

    let term: (NaiveDateTime, NaiveDateTime) = term.into();

    questions
        .filter(
            sql::<Bool>(flag.into())
                .and(user_id.eq(_user_id))
                .and(created_at.between(term.0, term.1)),
        )
        .order(sql::<Text>(sort.into()).desc())
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load::<Question>(conn)
        .map_err(|e| e.into())
}

pub fn paginate_users_answers(
    conn: &MysqlConnection,
    _user_id: u32,
    page: i64,
    sort: QuerySort,
    term: QueryTerm,
) -> Result<Vec<Question>> {
    use crate::schema_additional::answers::dsl::*;
    use crate::schema_additional::questions;

    let term: (NaiveDateTime, NaiveDateTime) = term.into();

    answers
        .filter(user_id.eq(_user_id).and(created_at.between(term.0, term.1)))
        .inner_join(questions::table)
        .order(sql::<Text>(sort.into()).desc())
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .select(questions::all_columns)
        .load(conn)
        .map_err(|e| e.into())
}

pub fn search(
    conn: &MysqlConnection,
    page: i64,
    flag: QueryFlag,
    sort: QuerySort,
    term: QueryTerm,
    q: &str,
) -> Result<Vec<Question>> {
    use crate::schema_additional::questions::dsl::*;

    let term: (NaiveDateTime, NaiveDateTime) = term.into();

    questions
        .filter(
            title
                .like(format!("%{}%", q))
                .or(body.like(format!("%{}%", q)))
                .and(created_at.between(term.0, term.1)),
        )
        .filter(sql::<Bool>(flag.into()))
        .order(sql::<Text>(sort.into()).desc())
        .limit(PAGE_SIZE)
        .offset(page * PAGE_SIZE)
        .load::<Question>(conn)
        .map_err(|e| e.into())
}

pub fn by_id(conn: &MysqlConnection, question_id: u32) -> Result<Option<Question>> {
    use crate::schema_additional::questions::dsl::*;

    questions
        .filter(id.eq(question_id))
        .limit(1)
        .load::<Question>(conn)
        .map_err(|e| e.into())
        .map(|mut q| {
            if q.len() > 0 {
                Some(q.pop().unwrap())
            } else {
                None
            }
        })
}
