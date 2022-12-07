use std::{collections::{HashSet, HashMap}, clone};

use anyhow::{Result, anyhow};

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
    ///Registers commands from a type that implements `CommandModule`
    /// 
    /// If a command has already been registered this method will return `Err`
    pub fn add_module<T: CommandModule>(&mut self) -> Result<()> {
        let commands = T::init();
        for command in commands {
            let command_name = command.name.clone();
            let group = command.group.clone().unwrap_or("".to_owned());
            if !self.commands.insert(command) {
                return Err(anyhow!("command named: {} in group: {} already exists!", 
                                command_name, group));
            }
        }
        Ok(())
    }
    pub fn remove_command(&mut self, name: &str, group: Option<String>) -> Result<()> {
        self.commands.remove(&Command {
            name: name.into(),
            desc: None,
            group: group,
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

        let first = args.get(0)
            .ok_or_else(|| anyhow!("First argument not found! (wtf?)"))?
            .to_owned();

        let valid = self.commands.iter()
                    .filter(|c| c.group == Some(first.clone()))
                    .collect::<Vec<&Command>>();

        if !valid.is_empty() {
            let name = args.get(1)
                .ok_or_else(|| anyhow!("No function specified"))?
                .to_owned();
            let valid = valid.into_iter()
                .filter(|c| c.name == name)
                .collect::<Vec<&Command>>();

            let command = valid.first()
                                    .ok_or_else(|| anyhow!("Command not found!"))?;
            args.remove(0);
            args.remove(0);
            if let Some(count) = command.args_num {
                if args.len() != count {
                    return Err(anyhow::anyhow!("Invalid argument count! (expected {})", 
                                                                            count))
                }
            }
            let mut context = CommandContext {
                manager: &mut self.manager,
                args: args,
            };
            (command.function)(&mut context)
        } else {
            let name = first;
            let commands = self.commands.iter()
                        .filter(|c| c.name == name)
                        .collect::<Vec<&Command>>();
            let command = commands.first()
                        .ok_or_else(|| anyhow!("Command not found!"))?;
            args.remove(0);

            if let Some(count) = command.args_num {
                if args.len() != count {
                    return Err(anyhow::anyhow!("Invalid argument count! (expected {})", 
                                                                            count))
                }
            }
            let mut context = CommandContext {
                manager: &mut self.manager,
                args: args,
            };
            (command.function)(&mut context)
        }
    }

    pub fn commands_info(&self) -> Vec<CommandInfo> {
        let mut ret = vec![];
        for cmd in &self.commands {
            let desc = cmd.desc.as_ref()
                .unwrap_or(&String::from(""))
                .clone();
            let group = cmd.group.clone();
                
            ret.push(CommandInfo {
                name: &cmd.name.as_str(),
                desc: desc,
                group: group,
                args: cmd.args_num.unwrap_or(0),
            });
        }
        ret.sort();
        ret
    }
}

pub struct Command {
    pub name: String,
    pub group: Option<String>,
    pub desc: Option<String>,
    pub args_num: Option<usize>,
    pub function: Box<dyn Fn(&mut CommandContext) -> Result<()>>
}
use std::hash::{Hash, Hasher};

use crate::EventsManager;
impl Hash for Command {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.group.hash(state);
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

#[derive(Ord, PartialEq, PartialOrd, Eq)]
pub struct CommandInfo<'a> {
    pub name: &'a str,
    pub desc: String,
    pub group: Option<String>,
    pub args: usize,
}


