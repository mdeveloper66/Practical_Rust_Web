use diesel::Queryable;
use serde::{Deserialize, Serialize};
use crate::schema::cats;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "cats"]
pub struct NewCat {
    // id will be added by the database
    pub name: String,
    pub image_path: String,
}
