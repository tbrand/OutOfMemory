use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct ListTag {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub user_id: u32,
    pub count_posts: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Debug)]
pub struct Query {
    page: Option<i64>,
    sort: Option<QuerySort>,
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
            .unwrap_or(QuerySort::Alphabet)
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
    #[serde(rename(deserialize = "alphabet"))]
    Alphabet,
    #[serde(rename(deserialize = "popular"))]
    Popular,
}

impl<'a> Into<&'a str> for QuerySort {
    fn into(self) -> &'a str {
        match self {
            QuerySort::Alphabet => "list_tags.name ASC",
            QuerySort::Popular => "list_tags.count_posts DESC",
        }
    }
}
