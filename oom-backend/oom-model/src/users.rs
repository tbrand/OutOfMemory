use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub avatar_url: String,
    pub token: Option<String>,
    pub token_expire: Option<NaiveDateTime>,
    pub access_token: Option<String>,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[table_name = "users"]
pub struct UpdateUser {
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub token: Option<Option<String>>,
    pub token_expire: Option<Option<NaiveDateTime>>,
    pub access_token: Option<Option<String>>,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub avatar_url: String,
    pub token: Option<String>,
    pub token_expire: Option<NaiveDateTime>,
    pub access_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
