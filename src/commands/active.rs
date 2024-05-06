use serenity::all::ActivityData;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn active(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    // Check if the user invoking the command is a bot owner
    if let Some(owner) = ctx.http.get_current_application_info().await?.owner {
        if owner.id != msg.author.id {
            return Ok(());
        }
    }

    let game = "forknife";
    ctx.set_activity(Some(ActivityData::playing(game)));

    Ok(())
}
