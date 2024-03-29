use bot_data::config::Config;
use bot_data::scramblr::get_scrambled_message;
use bot_data::user_message_cache::UserMessageCache;
use serenity::all::{CommandOptionType, ResolvedOption, ResolvedValue};
use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::user::User;


pub fn register() -> CreateCommand {
    CreateCommand::new("scramblr")
        .description("Scramble up your messages and make a new one!")
        .add_option(CreateCommandOption::new(
            CommandOptionType::User,
            "user",
            "The user to scramble your messages with"
        ).required(false)
    )
}

pub async fn run(msg_author: &User, user_message_cache: &UserMessageCache, options: &[ResolvedOption<'_>], config: &Config) -> String {
    // get user-provided user, or default to message author
    let provided_user = match options.get(0) {
        Some(ResolvedOption {
            value: ResolvedValue::User(mentioned_user, _),
            ..
        }) => mentioned_user,
        _ => msg_author
    };

    match get_scrambled_message(msg_author, &provided_user, user_message_cache, config) {
        Ok(content) => content,
        Err(scramblr_error) => format!("{}", scramblr_error.to_string())
    }
}