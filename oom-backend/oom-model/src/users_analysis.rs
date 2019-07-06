#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct UserAnalysis {
    pub count_answers: i64,
    pub count_is_best_answers: i64,
    pub count_questions: i64,
}
