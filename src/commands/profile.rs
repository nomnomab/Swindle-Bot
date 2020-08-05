use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};
use rusqlite::{Connection, params};

#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub xp: u32,
    pub level: u16,
    pub karma: u32,
    pub coins: u32
}

#[command]
pub fn profile(ctx: &mut Context, msg: &Message, _: Args) -> CommandResult {
    let author_id_raw = msg.author.id.0;
    let author_id = author_id_raw.to_string();

    let conn: &Connection;
    open_db!(ctx, conn);
    db_check!(conn, author_id, &0);
    //conn.execute("INSERT or IGNORE INTO users (id, xp, level, karma, coins) VALUES (?1, ?2, ?3, ?4, ?5)", params![author_id, &0, &0, &0, &0])?;

    let mut result = conn.prepare("SELECT xp, level, karma, coins FROM users u WHERE id = ?1")?;
    let users = result.query_map(params![author_id], |row| {
        let xp: u32 = row.get(0)?;
        let level: u16 = row.get(1)?;
        let karma: u32 = row.get(2)?;
        let coins: u32 = row.get(3)?;
        Ok(User { id: author_id_raw, xp, level, karma, coins })
    })?;

    for user in users {
        let user = user.unwrap();
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title(&msg.author.name);
                match &msg.author.avatar_url() {
                    Some(url) => e.thumbnail(url),
                    None => e.thumbnail("https://i.imgur.com/d7ted1B.png")
                };
                e.fields(vec![
                    ("xp", user.xp.to_string(), true),
                    ("level", user.level.to_string(), true),
                    ("karma", user.karma.to_string(), true),
                    ("coins", user.coins.to_string(), true)
                ]);

                e
            });

            m
        });
        break;
    }

    Ok(())
}