use crate::term_builder::TermBuilder;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct RankUser {
    pub id: u32,
    pub name: String,
    pub avatar_url: String,
    pub count_questions: i64,
    pub count_answers: i64,
    pub count_best_answers: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    page: Option<i64>,
    sort: Option<QuerySort>,
    term: Option<QueryTerm>,
}

impl Query {
    pub fn as_page(&self) -> i64 {
        self.page.as_ref().map(|p| *p).unwrap_or(0)
    }

    pub fn as_sort(&self) -> QuerySort {
        self.sort
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or(QuerySort::Alphabet)
    }

    pub fn as_term(&self) -> QueryTerm {
        self.term
            .as_ref()
            .map(|t| t.clone())
            .unwrap_or(QueryTerm::All)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum QuerySort {
    #[serde(rename(deserialize = "alphabet"))]
    Alphabet,
    #[serde(rename(deserialize = "questions"))]
    NumQuestions,
    #[serde(rename(deserialize = "answers"))]
    NumAnswers,
    #[serde(rename(deserialize = "best_answers"))]
    NumBestAnswers,
}

impl<'a> Into<&'a str> for QuerySort {
    fn into(self) -> &'a str {
        match self {
            QuerySort::Alphabet => "rank_users.name ASC",
            QuerySort::NumQuestions => "count_questions DESC",
            QuerySort::NumAnswers => "count_answers DESC",
            QuerySort::NumBestAnswers => "count_best_answers DESC",
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum QueryTerm {
    #[serde(rename(deserialize = "day"))]
    Day,
    #[serde(rename(deserialize = "week"))]
    Week,
    #[serde(rename(deserialize = "month"))]
    Month,
    #[serde(rename(deserialize = "all"))]
    All,
}

impl Into<(NaiveDateTime, NaiveDateTime)> for QueryTerm {
    fn into(self) -> (NaiveDateTime, NaiveDateTime) {
        match self {
            QueryTerm::Day => TermBuilder::new().days_ago(1).finish(),
            QueryTerm::Week => TermBuilder::new().weeks_ago(1).finish(),
            QueryTerm::Month => TermBuilder::new().days_ago(30).finish(),
            QueryTerm::All => TermBuilder::new().all().finish(),
        }
    }
}
