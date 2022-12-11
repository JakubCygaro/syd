use std::env::args;

use chrono::{Weekday, NaiveTime};
use syd::NewWeekEvent;
use syd::commands::{
    Command,
    CommandModule,
    CommandContext
};
use syd::models::transformed::{
    WeekEvent,
};
use anyhow::{
    anyhow,
    Result,
};
use syd_macros::*;


pub struct GeneralModule;

#[command_module]
impl GeneralModule {
    #[command]
    #[command_description("lists all database entries")]
    pub fn all(context: &mut CommandContext) -> Result<()> {

        let mut events = context.manager().get_all()?;
        events.sort_by(|a, b| 
            {
                a.day.num_days_from_monday().cmp(&b.day.num_days_from_monday())
            });
        events.print();
        Ok(())
    }
    #[command]
    #[command_description("day, name, is_lecture, start hour, end hour")]
    pub fn add(context: &mut CommandContext, 
        day: chrono::Weekday, 
        name: String, 
        is_lecture: bool,
        starth: chrono::NaiveTime,
        endh: chrono::NaiveTime) -> Result<()> {
        let new = NewWeekEvent{
            day: day,
            name: name,
            is_lecture: is_lecture,
            starth: starth,
            endh: endh
        };
        context.manager().add_event(new)?;
        println!("Event added successfuly!");
        Ok(())
    }
    #[command]
    pub fn delete(context: &mut CommandContext, id: i32) -> Result<()> {
        use std::io;
        println!("Really? [y/n]");
        let mut buff = "".to_owned();
        io::stdin().read_line(&mut buff)?;
        if buff.trim() == "y" {
            context.manager().delete_event(id)?;
            println!("Deleted successfully!");
        }
        Ok(())
    }
}

pub struct GetModule;

#[command_module]
#[command_group("get")]
impl GetModule {
    #[command]
    #[command_description("looks for an entry with provided id")]
    pub fn id(context: &mut CommandContext, id: i32) -> Result<()> {
        let event = context.manager().get_event(id)?;
        println!("{}", event);
        Ok(())
    }
    #[command]
    #[command_description("gets entries by day")]
    pub fn day(context: &mut CommandContext, day: chrono::Weekday) -> Result<()> {
        let mut ev = context.manager().by_day(day)?;
        ev.sort_by(|a, b| a.starth.cmp(&b.starth));
        ev.print();
        Ok(())
    }
    #[command]
    #[command_description("gets entries by start hour")]
    pub fn starth(context: &mut CommandContext, starth: chrono::NaiveTime) -> Result<()> {
        context.manager().by_starth(starth)?.print();
        Ok(())
    }
}

pub struct TestModule;

#[command_module]
impl TestModule {

    #[command]
    #[command_description("test command")]
    pub fn test(context: &mut CommandContext, val: i32) -> Result<()> {
        println!("{} * 2 = {}!", val, val * 2);
        Ok(())
    }
}

trait EventsExt {
    fn print(&self);
}

impl EventsExt for Vec<WeekEvent> {
    fn print(&self) {
        if self.is_empty() {
            println!("No events found!");
        } else {
            for e in self {
                println!("{}", e);
            }
        }
    }
}

