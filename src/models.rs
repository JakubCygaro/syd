use diesel::prelude::*;
pub use super::schema;
use schema::events;

pub mod transformed;

#[derive(Queryable)]
pub struct Event {
    pub id: Option<i32>,
    pub name: String,
    pub day: String,
    pub starth: String,
    pub endh: String,
    pub is_lecture: i32,
}
#[allow(non_snake_case)]
#[derive(Insertable, Clone)]
#[diesel(table_name = events)]
pub struct NewEvent {
    pub name: String,
    pub day: String,
    pub starth: String,
    pub endh: String,
    pub isLecture: i32,
}


