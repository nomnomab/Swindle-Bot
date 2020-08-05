#[macro_use]
mod databases;
mod commands;
mod events;
mod karma;

use serenity::{
    framework::{
        StandardFramework
    },
    prelude::*,
};

use serenity::Client;
use rusqlite::Connection;

use crate::commands::GENERAL_GROUP;
use databases::ConnectionWrapper;
use crate::databases::{ConfigWrapper, ConfigFile, ConfigMap};
use crate::events::Handler;

fn main() {
    let config_map = databases::load_config_files()
        .expect("Loading the config_map caused an error.");

    let startup_map = config_map
        .get(&ConfigFile::StartUp)
        .expect("The 'startup' map doesn't exist.");

    let token = startup_map.get("token")
        .expect("The token doesn't exist.");

    let mut prefix = startup_map.get("prefix")
        .expect("The prefix doesn't exist.")
        .clone();

    let mut client = Client::new(&token, Handler)
        .expect("Error creating the Client.");

    insert_data(&client, config_map);

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix(&prefix))
        .group(&GENERAL_GROUP));

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

fn insert_data(client: &Client, config_map: ConfigMap) {
    // db
    {
        let mut data = client.data.write();
        let wrapper = ConnectionWrapper::new(Connection::open("databases/main.db").unwrap());
        let mutex = Mutex::new(wrapper);
        data.insert::<ConnectionWrapper>(mutex);
    }

    // config_map
    {
        let mut data = client.data.write();
        let wrapper = ConfigWrapper::new(config_map);
        let mutex = Mutex::new(wrapper);
        data.insert::<ConfigWrapper>(mutex);
    }
}