use crate::schema::ups;
use chrono::NaiveDateTime;

#[derive(Insertable, Debug)]
#[table_name = "ups"]
pub struct NewUp {
    pub user_id: u32,
    pub post_id: u32,
    pub is_up: bool,
}

#[derive(Deserialize, Debug)]
pub struct NewUpBody {
    pub is_up: bool,
}

impl NewUpBody {
    pub fn as_new_up(self, user_id: u32, post_id: u32) -> NewUp {
        NewUp {
            user_id,
            post_id,
            is_up: self.is_up,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Up {
    pub user_id: u32,
    pub post_id: u32,
    pub is_up: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct IsUp {
    pub is_up: Option<bool>,
}
