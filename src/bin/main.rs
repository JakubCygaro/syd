use diesel::RunQueryDsl;
use syd::{*, models::{NewEvent, Event}, schema::events};
use chrono::NaiveTime;
fn main(){
    let mut manager = EventsManager::default().unwrap();

    use chrono::Weekday;

    manager.add_events(vec![
        NewWeekEvent::new("Czarna magia".into(), 
            Weekday::Fri, 
            "10:00:00", 
            "12:00:00", 
            false),
        NewWeekEvent::new("Lichwa".into(), 
            Weekday::Wed, 
            "13:30:00", 
            "15:00:00", 
            true),
        NewWeekEvent::new("Wróżbiarstwo".into(), 
            Weekday::Thu, 
            "20:00:00", 
            "22:00:00", 
            false),
    ]).unwrap();
        
    let revents = manager.get_all().unwrap();
    for e in revents {
        println!("{:?}", e)
    }
}