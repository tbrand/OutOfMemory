use crate::schema::posts;
use chrono::NaiveDateTime;

#[derive(Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: u32,
    pub question_post_id: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct NewPostBody {
    pub title: String,
    pub body: String,
    pub question_post_id: Option<u32>,
}

impl NewPostBody {
    pub fn as_new_post(self, user_id: u32) -> NewPost {
        NewPost {
            title: self.title,
            body: self.body,
            user_id,
            question_post_id: self.question_post_id,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct NewPostResponse {
    pub id: u32,
}

#[derive(Deserialize, Debug)]
pub struct UpdateContent {
    pub title: Option<String>,
    pub body: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateBestAnswer {
    pub is_best_answer: bool,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[table_name = "posts"]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub is_best_answer: Option<bool>,
}

#[derive(Queryable, Debug)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub is_best_answer: bool,
    pub user_id: u32,
    pub question_post_id: Option<u32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
