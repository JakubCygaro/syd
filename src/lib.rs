#[cfg(test)]
mod tests;

pub mod models;
pub mod schema;
pub mod commands;

use diesel::prelude::*;
use anyhow::{
    Result, Ok,
};
use diesel::sqlite::SqliteConnection;
use models::*;
pub use models::transformed::*;
pub use schema::events::{
    self,
    dsl::*,
};


pub struct EventsManager {
    connection: SqliteConnection,
}

impl EventsManager {
    pub fn default() -> Result<Self> {
        use dotenvy::dotenv;
        use std::env;
        dotenv()?;
        let database_url = env::var("DATABASE_URL")?;
        Ok(Self { connection: Self::establish_connection(&database_url)? })
    }
    pub fn custom(database_url: &str) -> Result<Self> {
        Ok(Self {
            connection: Self::establish_connection(database_url)?
        })
    }
    fn establish_connection(database_url: &str) -> Result<SqliteConnection> {
        let connection = SqliteConnection::establish(database_url)?;
        Ok(connection)
    }
    pub fn add_event(&mut self, new_event: NewWeekEvent) -> Result<()>{
        let new_event: NewEvent = new_event.into();
        diesel::insert_into(events::table)
            .values(new_event)
            .execute(&mut self.connection)?;
        Ok(())
    }
    pub fn delete_event(&mut self, event_id: i32) -> Result<()> {
        diesel::delete(events.filter(id.eq(event_id)))
            .execute(&mut self.connection)?;
        Ok(())
    }
    pub fn get_event(&mut self, event_id: i32) -> Result<WeekEvent> {
        let event = events.filter(id.eq(Some(event_id)))
            .first::<Event>(&mut self.connection)?;
        Ok(event.into())
    }
    pub fn get_events(&mut self, event: WeekEvent) -> Result<Vec<WeekEvent>>{
        let event: Event = event.into();
        let found = events
            .filter(id.eq(event.id))
            .or_filter(name.eq(event.name))
            .or_filter(day.eq(event.day))
            .or_filter(starth.eq(event.starth))
            .or_filter(endh.eq(event.endh))
            .or_filter(isLecture.eq(event.is_lecture))
            .load::<Event>(&mut self.connection)?;

        let ret: Vec<WeekEvent> = found.into_iter()
        .map(|e| e.into())
        .collect();

        Ok(ret)
    }
    pub fn get_all(&mut self) -> Result<Vec<WeekEvent>> {
        let res = 
            events.load::<Event>(&mut self.connection)?;

        let res = res.into_iter()
            .map(|e| e.into())
            .collect::<Vec<WeekEvent>>();
        Ok(res)
    }
    pub fn add_events(&mut self, event_s: Vec<NewWeekEvent>) -> Result<()> {
        let event_s: Vec<NewEvent> = event_s.into_iter()
            .map(|e| e.into())
            .collect();

        diesel::insert_into(events::table)
            .values(event_s)
            .execute(&mut self.connection)?;

        Ok(())
    }

}




