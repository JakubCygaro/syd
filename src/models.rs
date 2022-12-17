use diesel::prelude::*;
pub use super::schema;
use schema::events;
use diesel;

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

#[allow(non_snake_case)]
#[derive(Debug, Clone, AsChangeset, Identifiable)]
#[diesel(table_name = events)]
pub struct UpdatedWeekEvent {
    pub id: i32,
    pub name: Option<String>,
    pub day: Option<String>,
    pub starth: Option<String>,
    pub endh: Option<String>,
    pub isLecture: Option<i32>,
}

