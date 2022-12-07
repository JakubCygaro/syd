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
    #[command_args(0)]
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
    #[command_args(5)]
    #[command_description("day, name, is_lecture, start hour, end hour")]
    pub fn add(context: &mut CommandContext) -> Result<()> {

        let args = context.args();
        let new = NewWeekEvent{
            day: args[0].parse()?,
            name: args[1].to_owned(),
            is_lecture: args[2].parse()?,
            starth: NaiveTime::parse_from_str(&args[3], "%H:%M")?,
            endh: NaiveTime::parse_from_str(&args[4], "%H:%M")?
        };
        context.manager().add_event(new)?;
        println!("Event added successfuly!");
        Ok(())
    }
    #[command_args(1)]
    pub fn delete(context: &mut CommandContext) -> Result<()> {
        let id = context.args()[0].parse::<i32>()?;
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
    #[command_args(1)]
    #[command_description("looks for an entry with provided id")]
    pub fn id(context: &mut CommandContext) -> Result<()> {
        for arg in context.args() {
            println!("{}", arg);
        }
        let id: i32 = context
            .args()[0]
            .to_owned()
            .parse()?;
        let event = context.manager().get_event(id)?;
        println!("{}", event);
        Ok(())
    }
    #[command_args(1)]
    #[command_description("gets entries by day")]
    pub fn day(context: &mut CommandContext) -> Result<()> {
        use chrono::Weekday;

        let day = context.args().get(0).unwrap();
        let day = day.parse::<Weekday>()?;
        let mut ev = context.manager().by_day(day)?;
        ev.sort_by(|a, b| a.starth.cmp(&b.starth));
        ev.print();
        Ok(())
    }
    #[command_args(1)]
    #[command_description("gets entries by start hour")]
    pub fn starth(context: &mut CommandContext) -> Result<()> {
        use chrono::NaiveTime;
        let starth = context.args()
                                    .get(0)
                                    .unwrap();
        let starth = NaiveTime::parse_from_str(starth, "%H:%M:%S")?;
        context.manager().by_starth(starth)?.print();
        Ok(())
    }
}

pub struct TestModule;

#[command_module]
impl TestModule {

    #[command_description("test command")]
    pub fn test(context: &mut CommandContext) -> Result<()> {
        println!("Working!");
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

