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
    pub fn dupa(context: &mut CommandContext, dupa: i32, dupa2: i32, dupa3: i32) -> Result<()> {
        Ok(())
    }
    #[command]
    pub fn test(context: &mut CommandContext) -> Result<()>{
        Ok(())
    }
}
