use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use rusqlite::{Connection, params};
use crate::databases::{ConfigMap, ConfigFile};

#[command]
pub fn about(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    let config: &ConfigMap;
    open_config!(ctx, config);

    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            let about_config = config
                .get(&ConfigFile::About)
                .expect("");

            e.title(about_config
                .get("title")
                .expect(""));
            e.thumbnail(about_config
                .get("thumbnail")
                .expect(""));
            let version = config
                .get(&ConfigFile::StartUp)
                .expect("")
                .get("version")
                .expect("");
            e.footer(|f| {
                f.text(&format!("v{}", version));
                f
            });
            let content = about_config
                .get("content")
                .expect("");
            e.description(content);
            e
        });
        m
    });

    Ok(())
}