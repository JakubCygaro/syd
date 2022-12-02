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

        let events = context.manager().get_all()?;
        for e in events {
            println!("{}", e);
        }
        Ok(())
    }
    
    #[command_args(1)]
    #[command_description("looks for an entry with provided id")]
    pub fn get(context: &mut CommandContext) -> Result<()> {
        let id: i32 = context
            .args()[0]
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

    #[command_description("test command")]
    pub fn test(context: &mut CommandContext) -> Result<()> {
        println!("Working!");
        Ok(())
    }
}