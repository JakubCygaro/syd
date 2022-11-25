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

pub struct GeneralModule;

impl CommandModule for GeneralModule {
    fn init() -> Vec<Command> {
        let mut commands = vec![];
        commands.push(Command {
            name: "all".into(),
            args_num: Some(0),
            function: Box::new(Self::all),
        });

        commands
    }
}

impl GeneralModule {
    fn all(context: &mut CommandContext, _args: Vec<String>) -> Result<()> {

        let events = context.manager().get_all()?;
        for e in events {
            println!("{}", e);
        }
        Ok(())
    }
    // fn today(context: &mut CommandContext, _args: Vec<String>) -> Result<()> {

        

    //     let events = context.manager()
    //             .get_events(WeekEvent 
    //                 { 
    //                     id: None, 
    //                     name: "".into(), 
    //                     day: , 
    //                     starth: (), 
    //                     endh: (), 
    //                     is_lecture: () 
    //                 })?;
    //     Ok(())
    // }
}