use serenity::{
    model::{gateway::Ready},
    prelude::*,
};
use serenity::model::prelude::Message;
use crate::ConfigFile;
use rusqlite::{Connection, params, named_params};
use crate::databases::ConfigMap;
use crate::karma::*;

pub struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let config: &ConfigMap;
        open_config!(ctx, config);

        let prefix = config
            .get(&ConfigFile::StartUp)
            .expect("The 'startup' map doesn't exist.")
            .get("prefix")
            .expect("The prefix doesn't exist.");

        if msg.content.starts_with(prefix) {
            return;
        }

        if check_for_karma(&msg) {
            let mentions = msg.mentions;
            for men in mentions {

            }
            return;
        }

        let author_id = msg.author.id.0.to_string();

        let conn: &Connection;
        open_db!(ctx, conn);

        conn.execute("INSERT or IGNORE INTO users (id, xp, level) VALUES (?1, ?2, ?3)", params![author_id, &0, &0]).unwrap();

        let mut result = conn.prepare("SELECT xp FROM users WHERE id = :id;").unwrap();
        let mut rows = result.query_named(named_params!{":id": author_id}).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let mut xp: u32 = row.get(0).unwrap();
            xp += 1;
            conn.execute("UPDATE users SET xp = (?1) WHERE id = (?2);", params![xp.to_string(), author_id]).unwrap();
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}