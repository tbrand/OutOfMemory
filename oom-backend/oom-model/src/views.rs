use crate::schema::views;

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[table_name = "views"]
pub struct NewView {
    pub user_id: u32,
    pub post_id: u32,
}
