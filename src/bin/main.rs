use syd_macros::*;
use syd::commands::*;
use syd::*;
use anyhow::*;
fn main(){
}

struct Module;

#[command_module]
impl Module {
    #[command]
    pub fn test(context: &mut CommandContext, arg1: i32, arg2: i32) -> Result<()>{
        Ok(())
    }
}
