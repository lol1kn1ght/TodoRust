use serenity::framework::standard::CommandResult;
use std::collections::HashMap;

pub type Command = fn(String) -> CommandResult;
pub type Commands = HashMap<String, Command>;
