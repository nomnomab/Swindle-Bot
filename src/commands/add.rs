use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub fn add(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let a = args.single::<f64>().unwrap();
    let b = args.single::<f64>().unwrap();
    let sum = a + b;
    let _ = msg.channel_id.say(&ctx.http, sum);

    Ok(())
}