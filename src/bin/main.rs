use diesel::RunQueryDsl;
use syd::{*, models::{NewEvent, Event}, schema::events};
use chrono::NaiveTime;
fn main(){
    let mut manager = EventsManager::default().unwrap();

    
    manager.add_events(vec![
        NewWeekEvent::new("Czarna magia".into(), 
            Day::Friday, 
            "10:00:00", 
            "12:00:00", 
            false),
        NewWeekEvent::new("Lichwa".into(), 
            Day::Wednesday, 
            "13:30:00", 
            "15:00:00", 
            true),
        NewWeekEvent::new("Wróżbiarstwo".into(), 
            Day::Thursday, 
            "20:00:00", 
            "22:00:00", 
            false),
    ]).unwrap();
        
    let revents = manager.get_all().unwrap();
    for e in revents {
        println!("{:?}", e)
    }
}