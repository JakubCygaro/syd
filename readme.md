# SYD
A tool for querying into a database of my university week timetable or smth

## TODO
Implement automatic command arguments detection and parsing via a macro-written custom parsing function, funtion parameters would have to implement a trait that allows parsing an argument from a string into the desired type like so:

```rust
trait ArgParse {
    fn arg_parse(string: &str) -> Result<Self>;
}
impl ArgParse for i32 {
    fn arg_parse(string: &str) -> Result<Self> {
        string.parse()?
    }
}

#[command]
pub fn command(context: &mut CommandContext, id: i32) -> Result<()>;

//parsing function that would be inside the Command struct
pub fn parse_cmd_args(&self, context: &mut CommandContext, args: Vec<String>) -> Result<()> {
    if args.len() != 1 {
        return Err(anyhow!("invalid argument count!"));
    }
    let arg0 = <i32 as ArgParse>::arg_parse(arg[0])?;
    //... other args here

    Self::command(&mut context, arg0)?
}
```
I would have to add a `#[command]` macro that would tell the `#[command_module]` macro what
methods are supposed to actually be made into commands. Methods with the `#[command]` macro
would then be analyzed and if all their parameters implement the `ArgParse` trait, a parsing
function would be written that actually calls the target method of the command.

```rust
struct Command {
    pub name: String,
    pub group: Option<String>,
    pub args_num: Option<usize>, // <- this would probably need to go
    pub parse: Box<dyn Fn(&mut CommandContext, Vec<String>) -> Result<()>>, // <- this is   where the command arguments are parsed and the function called.
}
```
As a result the `#[command_ars]` macro would become obsolete