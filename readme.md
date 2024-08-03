# SYD
A tool for querying into a database of my weekly university schedule. (No longer in use)

## Diesel
This thing was originaly planned as a playground for learning diesel but at the moment like 80% of the effort was spent writing macros, so actual client-side features are lacking at the moment.

## Command framework
This thing uses custom macros to enable fast command writing without the need for writing my own parsing code.

## Example

The `command_module` macro implemets the `CommandModule` trait for a chosen struct, by finding all methods decorated with the `command` attribute. The `command` macro handles all validation.
 
```rust
#[command_module]
impl Module {
    #[command]
    #[command_description("this is a test")]
    pub fn test(context: &mut CommandContext, arg1: i32, arg2: i32) -> Result<()>{
        Ok(())
    }
}
```
`CommandContext` is a required argument for a command function as it provides access to ORM features of this program.

You can also use the `command_group` macro to specify that a command must be preceeded by the name of a group that it belongs to:

```rust
#[command]
#[command_group("kwas")]
pub fn test(context: &mut CommandContext) -> Result<()> {
    println!("Hello!");
    Ok(())
}
```
To call this command through a `CommandHandler` you'd have to type: 
`kwas test`, not just `test`.
