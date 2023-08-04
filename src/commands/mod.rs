pub mod test;

use crate::structs::{Command, Commands};
use serenity::framework::standard::CommandResult;
use std::collections::HashMap;

use test::testCommand;

pub async fn register(commands: &mut Commands) {
    commands.insert(String::from("test"), testCommand);
}

// if let Some(command_function) = command_map.get("my_command") {
//     match command_function("Some Name".to_string()) {
//         Ok(_) => println!("Команда успешно выполнена!"),
//         Err(error) => println!("Ошибка при выполнении команды: {}", error),
//     }
// } else {
//     println!("Команда не найдена в HashMap.");
// }
