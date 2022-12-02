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
    pub fn all(context: &mut CommandContext) -> Result<()> {

        let events = context.manager().get_all()?;
        for e in events {
            println!("{}", e);
        }
        Ok(())
    }
    
    #[command_args(1)]
    pub fn get(context: &mut CommandContext) -> Result<()> {
        let id: i32 = context
            .args()
            .get(0)
            .ok_or_else(|| anyhow!("ivalid argument count"))?
            .to_owned()
            .parse()?;
        let event = context.manager().get_event(id)?;
        println!("{}", event);
        Ok(())
    }
}

pub struct TestModule;

#[command_module]
impl TestModule {
    pub fn test(context: &mut CommandContext) -> Result<()> {
        println!("Working!");
        Ok(())
    }
}