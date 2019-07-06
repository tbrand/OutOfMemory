use crate::schema::posts_tags;
use crate::schema::tags;
use crate::schema::users_tags;
use chrono::NaiveDateTime;

#[derive(Insertable, Debug)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub description: String,
    pub user_id: u32,
}

#[derive(Deserialize, Debug)]
pub struct NewTagBody {
    pub name: String,
    pub description: String,
}

impl NewTagBody {
    pub fn as_new_tag(self, user_id: u32) -> NewTag {
        NewTag {
            name: self.name,
            description: self.description,
            user_id,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub user_id: u32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "posts_tags"]
pub struct LinkPost {
    pub post_id: u32,
    pub tag_id: u32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users_tags"]
pub struct LinkUser {
    pub user_id: u32,
    pub tag_id: u32,
}
