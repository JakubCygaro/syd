use std::io::Read;
pub mod modules;

use syd::commands::*;
use syd::*;

fn main() {
    let manager = EventsManager::default().unwrap();
    let mut handler = CommandHandler::new(manager);
    handler.add_module::<modules::GeneralModule>().unwrap();
    handler.add_module::<modules::TestModule>().unwrap();
    use std::io;
    startup_message();
    loop {
        let mut buff = String::from("");
        io::stdin().read_line(&mut buff).unwrap();
        let buff = buff.trim();
        match buff {
            ".quit" => break,
            ".commands" => {
                for (n, d) in handler.commands_names() {
                    println!("{} -> {}", n, d.clone().unwrap_or("no description".to_owned()));
                }
            },
            _ => handler.handle(buff.into()).or_else(|e| {
                    println!("{:?}", e);
                    Ok::<_, &str>(())
                }).unwrap_or_else(|err| println!("{}", err)),
        }
    }
}

fn startup_message() {
    println!("==|SYD 0.1|==");
    println!("type `.quit` to exit the program.");
    println!("type `.commands` to get all commands.");
}