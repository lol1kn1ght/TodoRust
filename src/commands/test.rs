use core::result::Result::Ok;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use std::pin::Pin;

pub fn testCommand(name: String) -> CommandResult {
    println!("Test command");
    Ok(())
}
