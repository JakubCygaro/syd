use std::io::Read;
pub mod modules;

use syd::commands::*;
use syd::*;

fn main() {
    let manager = EventsManager::default().unwrap();
    let mut handler = CommandHandler::new(manager);
    handler.add_module::<modules::GeneralModule>();
    handler.add_module::<modules::TestModule>();
    // handler.add_command(Command {
    //     name: "id".into(),
    //     function: Box::new(|context,args|{
    //         if args.len() != 1 {
    //             return Err(anyhow::anyhow!("Invalid parameter count!"))
    //         }
    //         let event_id: i32 = args.get(0)
    //                                 .unwrap()
    //                                 .to_owned()
    //                                 .trim()
    //                                 .parse()?;
    //         let ev = context.manager()
    //                                 .get_event(event_id)?;
    //         println!("{:?}", ev);
    //         Ok(())
    //     }),
    // }).unwrap();

    use std::io;
    let mut buff = String::from("");
    println!("type...");
    io::stdin().read_line(&mut buff).unwrap();
    let buff = buff.trim();

    handler.handle(buff.into()).or_else(|e| {
        println!("{:?}", e);
        Ok::<_, &str>(())
    }).unwrap();

}