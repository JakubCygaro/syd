pub mod modules;

use syd::commands::*;
use syd::*;

fn main() {
    let manager = EventsManager::default().unwrap();
    let mut handler = CommandHandler::new(manager);
    handler.add_module::<modules::GeneralModule>().unwrap();
    handler.add_module::<modules::TestModule>().unwrap();
    handler.add_module::<modules::GetModule>().unwrap();
    use std::io;
    startup_message();
    loop {
        println!("type a command...");
        let mut buff = String::from("");
        io::stdin().read_line(&mut buff).unwrap();
        let buff = buff.trim();
        match buff {
            ".quit" => break,
            ".commands" => {
                for inf in handler.commands_info() {
                    print_command_info(&inf);
                }
            },
            _ => {
                println!();
                handler.handle(buff.into()).or_else(|e| {
                    println!("{:?}", e);
                    Ok::<_, &str>(())
                }).unwrap_or_else(|err| println!("{}", err))
            },
        }
    }
}

fn startup_message() {
    println!("==|SYD 1.0|==");
    println!("type `.quit` to exit the program.");
    println!("type `.commands` to get all commands.");
}

fn print_command_info(info: &syd::commands::CommandInfo) {
    println!();
    println!("Description: {}", info.desc);
    if let Some(g) = &info.group {
        print!("{} ", g);
    }
    print!("{} ", info.name);
    
    let mut sig = "(".to_owned();
    for i in &info.args {
        sig.push_str(format!("{}: {}, ", i.name, i.ty).as_str());
    }
    if let Some(index) = sig.rfind(','){
        sig.remove(index);
    }
    print!("{}", sig);
    print!(")");
    println!();
}