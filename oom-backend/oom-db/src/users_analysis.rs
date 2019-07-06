use crate::error::Result;
use crate::model::users_analysis::*;
use diesel::dsl::sql;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::sql_types::BigInt;

pub fn analysis(conn: &MysqlConnection, _user_id: u32) -> Result<UserAnalysis> {
    use crate::schema::posts::dsl::*;

    posts
        .select((
            sql::<BigInt>("IFNULL(SUM(ISNULL(posts.question_post_id)), 0) as count_questions"),
            sql::<BigInt>("IFNULL(SUM(!ISNULL(posts.question_post_id)), 0) as count_answers"),
            sql::<BigInt>("IFNULL(SUM(posts.is_best_answer), 0) as count_best_answers"),
        ))
        .filter(user_id.eq(_user_id))
        .load(conn)
        .map_err(|e| e.into())
        .map(|mut p| p.pop().unwrap())
}
