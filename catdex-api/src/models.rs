use serde::{Serialize, Deserialize};
use validator_derive::Validate;


#[derive(Serialize, Queryable)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub image_path: String,
}


#[derive(Deserialize, Validate)]
pub struct CatEndpointPath {
    #[validate(range(min=1, max=150))]
    pub id:i32
}

