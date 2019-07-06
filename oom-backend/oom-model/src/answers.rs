use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Answer {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub is_best_answer: bool,
    pub user_id: u32,
    pub question_post_id: u32,
    pub count_ups: i64,
    pub count_downs: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    page: Option<i64>,
    sort: Option<QuerySort>,
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
}

#[derive(Deserialize, Clone, Debug)]
pub enum QuerySort {
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
            QuerySort::Hot => "count_ups",
            QuerySort::New => "created_at",
            QuerySort::Active => "updated_at",
        }
    }
}
