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
use syd_macros::command_module;


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
    pub fn dupa(context: &mut CommandContext) -> Result<()> {
        println!("CHUJ");
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