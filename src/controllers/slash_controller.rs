use serenity::model::prelude::application_command::ApplicationCommandInteraction;

use crate::structs::Command;

pub struct Controller {}

impl Controller {
    pub fn register(name: String, command: Command) {}
    pub fn handle_command(interaction: &ApplicationCommandInteraction) {
        println!("Handling a /{} command", interaction.data.name);
    }
}
