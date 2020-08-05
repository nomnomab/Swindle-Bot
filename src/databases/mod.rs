use rusqlite::Connection;
use serenity::prelude::{TypeMapKey, Mutex};
use std::collections::HashMap;
use std::{io, fs};
use std::path::PathBuf;
use std::ffi::OsStr;

pub type ConfigMap = HashMap<ConfigFile, HashMap<String, String>>;

#[derive(Hash, Eq, PartialEq)]
pub enum ConfigFile {
    StartUp,
    About
}

impl ConfigFile {
    fn convert(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "startup" => ConfigFile::StartUp,
            "about" => ConfigFile::About,
            _ => panic!("{} does not have a ConfigFile enum equivalence.", name)
        }
    }
}

pub struct ConnectionWrapper {
    pub value: Connection
}

impl ConnectionWrapper {
    pub fn new(value: Connection) -> Self {
        Self {
            value
        }
    }
}

impl TypeMapKey for ConnectionWrapper {
    type Value = Mutex<ConnectionWrapper>;
}

pub struct ConfigWrapper {
    pub value: ConfigMap
}

impl ConfigWrapper {
    pub fn new(value: ConfigMap) -> Self {
        Self {
            value
        }
    }
}

impl TypeMapKey for ConfigWrapper {
    type Value = Mutex<ConfigWrapper>;
}

macro_rules! open_db {
    ($ctx: ident, $conn: ident) => {
        let data = $ctx.data.read();
        let wrapper = data
            .get::<crate::databases::ConnectionWrapper>()
            .unwrap();
        let conn = &wrapper.lock();
        $conn = &conn.value;
    }
}

macro_rules! open_config {
    ($ctx: ident, $config: ident) => {
        let data = $ctx.data.read();
        let wrapper = data
            .get::<crate::databases::ConfigWrapper>()
            .unwrap();
        let conn = &wrapper.lock();
        $config = &conn.value;
    }
}

pub fn load_config_files() -> Result<ConfigMap, io::Error> {
    let mut config_map = HashMap::new();

    let config_files: Vec<PathBuf> = fs::read_dir("./config/")?
        .map(|res| res.map(|e| e.path()))
        .filter(|path| path.as_ref().unwrap().extension().unwrap() == OsStr::new("cfg"))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for file in config_files {
        println!("Loading config from {:?}", &file);

        let contents = fs::read_to_string(&file)
            .expect(&format!("Contents could not be read from {:?}", &file));

        let new_line_split = contents
            .split('\n')
            .collect::<Vec<_>>();

        let mut current_map = HashMap::new();

        for line in new_line_split {
            let variable_split = line
                .split('=')
                .collect::<Vec<_>>();

            if variable_split.len() != 2 {
                continue;
            }

            current_map.insert(variable_split[0].to_string(), variable_split[1].trim_end().to_string());
        }

        let name = file.file_name().unwrap().to_str().unwrap().to_string();
        config_map.insert(ConfigFile::convert(&name[..(name.len()-4)]), current_map);
    }

    Ok(config_map)
}