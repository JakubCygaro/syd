use chrono::{Weekday, NaiveTime, Datelike};
use syd::NewWeekEvent;
use syd::commands::CommandContext;
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
    #[command_description("Lists all database entries.")]
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
    #[command_description("Add an entry to the timetable.")]
    pub fn add(context: &mut CommandContext, 
        day: Weekday, 
        name: String, 
        is_lecture: bool,
        starth: NaiveTime,
        endh: NaiveTime) -> Result<()> {
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
    #[command_description("Deletes an entry with the provided id")]
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
    #[command]
    #[command_description("Change an entry with given id")]
    pub fn change(context: &mut CommandContext, 
        id: i32,
        new_day: Option<Weekday>, 
        new_name: Option<String>,
        new_is_lecture: Option<bool>,
        new_statrh: Option<NaiveTime>,
        new_endh: Option<NaiveTime>
        ) 
        -> Result<()>
    {
        use syd::models::UpdatedWeekEvent;
        context.manager().change_event(UpdatedWeekEvent 
            { 
                id: id, 
                name: new_name, 
                day: match new_day {
                    Some(d) => Some(d.to_string()),
                    None => None,
                }, 
                starth: match new_statrh {
                    Some(s) => Some(s.to_string()),
                    None => None,
                }, 
                endh: match new_endh {
                    Some(e) => Some(e.to_string()),
                    None => None,
                }, 
                isLecture: match new_is_lecture {
                    Some(l) => Some(l as i32),
                    None => None
                } 
            })?;
        println!("Event changed!");
        println!("{}", context.manager().get_event(id)?);
        Ok(())
    }
    #[command]
    #[command_description("Get events of today")]
    pub fn today(context: &mut CommandContext) -> Result<()> {
        let now = chrono::Utc::now();
        let day = now.weekday();
        context.manager().by_day(day)?.print();
        Ok(())
    }
    #[command]
    #[command_description("Gets the closest x amount of events today")]
    pub fn near(context: &mut CommandContext, amount: u32) -> Result<()> {
        use chrono::Utc;
        let events = context.manager()
            .by_day(Utc::now().weekday())?;
        events.into_iter()
            .filter(|e| {
                e.starth > Utc::now().time()
            })
            .take(amount as usize)
            .collect::<Vec<WeekEvent>>()
            .print();
        Ok(())
    }

}

pub struct GetModule;

#[command_module]
#[command_group("get")]
impl GetModule {
    #[command]
    #[command_description("Looks for an entry with provided id.")]
    pub fn id(context: &mut CommandContext, id: i32) -> Result<()> {
        let event = context.manager().get_event(id)?;
        println!("{}", event);
        Ok(())
    }
    #[command]
    #[command_description("Gets entries by day.")]
    pub fn day(context: &mut CommandContext, day: Weekday) -> Result<()> {
        let mut ev = context.manager().by_day(day)?;
        ev.sort_by(|a, b| a.starth.cmp(&b.starth));
        ev.print();
        Ok(())
    }
    #[command]
    #[command_description("Gets entries by start hour")]
    pub fn starth(context: &mut CommandContext, starth: NaiveTime) -> Result<()> {
        context.manager().by_starth(starth)?.print();
        Ok(())
    }
    #[command]
    #[command_description("Gets entries by end hour.")]
    pub fn endh(context: &mut CommandContext, endh: NaiveTime) -> Result<()> {
        context.manager().by_endh(endh)?.print();
        Ok(())
    }
    #[command]
    #[command_description("Gets entries based on wether they are lectures.")]
    pub fn is_lecture(context: &mut CommandContext, is_lecture: bool) -> Result<()> {
        context.manager().by_is_lecture(is_lecture)?.print();
        Ok(())
    }
}

pub struct TestModule;

#[command_module]
impl TestModule {

    #[command]
    #[command_description("test command")]
    pub fn test(_context: &mut CommandContext, val: i32) -> Result<()> {
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

