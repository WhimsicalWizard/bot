use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::CommandOptionType;
use serenity::model::application::ResolvedOption;

pub fn run(options: &[ResolvedOption]) -> String {
    "retrn".to_string()
}
pub fn register() -> CreateCommand {
    CreateCommand::new("welcome")
        .description("Welcome a user")
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The user to welcome")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "message", "The message to send")
                .required(true)
                .add_string_choice_localized(
                    "Welcome to our cool server! Ask me if you need help",
                    "pizza",
                    [(
                        "de",
                        "Willkommen auf unserem coolen Server! Frag mich, falls du Hilfe brauchst",
                    )],
                ),
        )
}
