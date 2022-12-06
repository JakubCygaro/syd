use std::fmt::Display;

use crate::starth;

use super::*;
use schema::events;
use diesel::prelude::*;
use anyhow::Result;

use chrono::{
    self,
    Utc,
    DateTime,
    Weekday
};

#[derive(Clone, Debug)]
pub struct WeekEvent {
    pub id: Option<i32>,
    pub name: String,
    pub day: Weekday,
    pub starth: chrono::NaiveTime,
    pub endh: chrono::NaiveTime,
    pub is_lecture: bool,
}

impl From<Event> for WeekEvent {
    fn from(event: Event) -> Self {
        Self { 
            id: event.id, 
            name: event.name, 
            day: event.day.parse::<Weekday>().unwrap(),
            starth: chrono::NaiveTime::parse_from_str(&event.starth, "%H:%M:%S")
                    .unwrap(), 
            endh: chrono::NaiveTime::parse_from_str(&event.endh, "%H:%M:%S")
                    .unwrap(), 
            is_lecture: event.is_lecture != 0 
        }
    }
}

impl Into<Event> for WeekEvent {
    fn into(self) -> Event {
        Event { 
            id: self.id, 
            name: self.name, 
            day: self.day.to_string(), 
            starth: self.starth.to_string(), 
            endh: self.endh.to_string(), 
            is_lecture: self.is_lecture as i32 
        }
    }
}

// #[derive(Copy, Clone, Debug)]
// pub enum Day {
//     Monday = 1,
//     Tuesday = 2,
//     Wednesday = 3,
//     Thursday = 4, 
//     Friday = 5,
//     Saturday = 6,
//     Sunday = 7,
// }

// impl Into<i32> for Day {
//     fn into(self) -> i32 {
//         match self {
//             Day::Monday => 1,
//             Day::Tuesday => 2,
//             Day::Wednesday => 3,
//             Day::Thursday => 4,
//             Day::Friday => 5,
//             Day::Saturday => 6,
//             Day::Sunday => 7,
//         }
//     }
// }

// impl Into<Day> for i32 {
//     fn into(self) -> Day {
//         match self {
//             1 => Day::Monday,
//             2 => Day::Tuesday,
//             3 => Day::Wednesday,
//             4 => Day::Thursday,
//             5 => Day::Friday,
//             6 => Day::Saturday,
//             7 => Day::Sunday,
//             _ => panic!("Parsing day from i32 failed for value: {}", self)
//         }
//     }
// }

pub struct NewWeekEvent {
    pub name: String,
    pub day: Weekday,
    pub starth: chrono::NaiveTime,
    pub endh: chrono::NaiveTime,
    pub is_lecture: bool,
}
impl NewWeekEvent {
    pub fn new(
        name: String,
        day: Weekday,
        rstarth: &str,
        endh: &str,
        is_lecture: bool) -> Self {
            Self {
                day: day,
                name: name,
                starth: chrono::NaiveTime::parse_from_str(rstarth, "%H:%M:%S")
                        .unwrap(),
                endh: chrono::NaiveTime::parse_from_str(endh, "%H:%M:%S")
                        .unwrap(),
                is_lecture: is_lecture,
            }
    }
}

impl Into<NewEvent> for NewWeekEvent {
    fn into(self) -> NewEvent {
        NewEvent { 
            name: self.name, 
            day: self.day.to_string(), 
            starth: self.starth.to_string(), 
            endh: self.endh.to_string(), 
            isLecture: self.is_lecture as i32,
        }
    }
}

// impl Display for Day {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Day::Monday => write!(f, "Monday"),
//             Day::Tuesday => write!(f, "Tuesday"),
//             Day::Wednesday => write!(f, "Wednesday"),
//             Day::Thursday => write!(f, "Thursday"),
//             Day::Friday => write!(f, "Friday"),
//             Day::Saturday => write!(f, "Saturday"),
//             Day::Sunday => write!(f, "Sunday"),
//         }
//     }
// }

impl Display for WeekEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Day: {}\n", self.day)?;
        write!(f, "Event: {}\n", self.name)?;
        write!(f, "Is lecture?: {}\n", self.is_lecture)?;
        write!(f, "Starts at: {}\n", self.starth)?;
        write!(f, "Ends at: {}\n", self.endh)?;
        write!(f, "ID: {}\n", self.id.unwrap())
    }
}


