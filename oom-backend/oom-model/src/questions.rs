use crate::term_builder::TermBuilder;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Question {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub is_solved: bool,
    pub user_id: u32,
    pub count_ups: i64,
    pub count_downs: i64,
    pub count_views: i64,
    pub count_answers: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    page: Option<i64>,
    sort: Option<QuerySort>,
    flag: Option<QueryFlag>,
    term: Option<QueryTerm>,
    q: Option<String>,
}

impl Query {
    pub fn as_page(&self) -> i64 {
        self.page.as_ref().map(|p| *p).unwrap_or(0)
    }

    pub fn as_sort(&self) -> QuerySort {
        self.sort
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or(QuerySort::New)
    }

    pub fn as_flag(&self) -> QueryFlag {
        self.flag
            .as_ref()
            .map(|f| f.clone())
            .unwrap_or(QueryFlag::None)
    }

    pub fn as_term(&self) -> QueryTerm {
        self.term
            .as_ref()
            .map(|t| t.clone())
            .unwrap_or(QueryTerm::All)
    }

    pub fn as_q(&self) -> String {
        self.q
            .as_ref()
            .map(|q| q.to_owned())
            .unwrap_or("".to_owned())
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum QuerySort {
    #[serde(rename(deserialize = "popular"))]
    Popular,
    #[serde(rename(deserialize = "hot"))]
    Hot,
    #[serde(rename(deserialize = "new"))]
    New,
    #[serde(rename(deserialize = "active"))]
    Active,
}

impl<'a> Into<&'a str> for QuerySort {
    fn into(self) -> &'a str {
        match self {
            QuerySort::Popular => "count_views",
            QuerySort::Hot => "count_ups",
            QuerySort::New => "created_at",
            QuerySort::Active => "updated_at",
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum QueryFlag {
    #[serde(rename(deserialize = "unsolved"))]
    UnSolved,
    #[serde(rename(deserialize = "none"))]
    None,
}

impl<'a> Into<&'a str> for QueryFlag {
    fn into(self) -> &'a str {
        match self {
            QueryFlag::UnSolved => "questions.is_solved IS FALSE",
            QueryFlag::None => "questions.id > 0",
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
