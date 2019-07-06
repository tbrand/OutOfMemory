use crate::schema::comments;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};

#[derive(Insertable, Debug)]
#[table_name = "comments"]
pub struct NewComment {
    pub body: String,
    pub user_id: u32,
    pub post_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct NewCommentBody {
    pub body: String,
}

impl NewCommentBody {
    pub fn as_new_comment(self, user_id: u32, post_id: u32) -> NewComment {
        NewComment {
            body: self.body,
            user_id,
            post_id,
        }
    }
}

#[derive(Queryable, Serialize, Debug)]
pub struct Comment {
    pub id: u32,
    pub body: String,
    pub user_id: u32,
    pub post_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
