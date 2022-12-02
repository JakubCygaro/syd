use std::collections::{HashSet};

use anyhow::Result;

pub struct CommandHandler {
    commands: HashSet<Command>,
    manager: EventsManager
}

impl CommandHandler {
    pub fn new(manager: EventsManager) -> Self {
        Self {
            commands: HashSet::new(),
            manager: manager
        }
    }
    pub fn add_command(&mut self, command: Command) -> Result<()> {
        let name = command.name.clone();
        if name.contains(' ') {
            return Err(anyhow::anyhow!("Invalid command name!"));
        }
        if !self.commands.insert(command) {
            return Err(anyhow::anyhow!("Command {} already registered!", &name));
        }
        Ok(())
    }
    pub fn add_module<T: CommandModule>(&mut self) -> Result<()> {
        let commands = T::init();
        for command in &commands {
            if self.commands.contains(command) {
                return Err(anyhow::anyhow!("Command with name {} already registered!", 
                    &command.name));
            }
        }
        self.commands.extend(commands.into_iter());
        Ok(())
    }
    pub fn remove_command(&mut self, name: &str) -> Result<()> {
        self.commands.remove(&Command {
            name: name.into(),
            args_num: None,
            function: Box::new(|_|{Ok(())})
        });
        Ok(())
    }
    pub fn handle(&mut self, input: String) -> Result<()> {
        let mut args = input.split_ascii_whitespace()
            .map(|a| a.to_owned())
            .collect::<Vec<String>>();
        if args.is_empty() {
            return Err(anyhow::anyhow!("No arguments found in input stream!"));
        }
        let name = &args.get(0).unwrap().to_owned();

        for command in &self.commands {
            if &command.name == name {
                args.remove(0);
                if let Some(count) = command.args_num {
                    if args.len() != count {
                        return Err(anyhow::anyhow!("Invalid argument count!"))
                    }
                }
                let mut context = CommandContext {
                    manager: &mut self.manager,
                    args: args,
                };
                (command.function)(&mut context)?;
                return Ok(());
            }
        }
        Err(anyhow::anyhow!("Command not found!"))
    }
    pub fn commands_names(&self) -> Vec<&str> {
        self.commands.iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<&str>>()
    }
}


pub struct Command {
    pub name: String,
    pub args_num: Option<usize>,
    pub function: Box<dyn Fn(&mut CommandContext) -> Result<()>>
}
use std::hash::{Hash, Hasher};

use crate::EventsManager;
impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Command {}

pub struct CommandContext<'a> {
    manager: &'a mut EventsManager,
    args: Vec<String>,
}
impl<'a> CommandContext<'a> {
    pub fn manager(&mut self) -> &mut EventsManager {
        &mut self.manager
    }
    pub fn args(&mut self) -> &mut Vec<String> {
        &mut self.args
    }
}

pub trait CommandModule {
    fn init() -> Vec<Command>;
}