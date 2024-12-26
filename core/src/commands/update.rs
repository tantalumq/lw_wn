use core::{Config, Error};
use std::{fs::File, str::FromStr};

use uuid::Uuid;

use super::FILENAME;

pub fn execute(uuid: &str, new_name: &str, new_path: &str) -> Result<(), Error> {
    let uuid = Uuid::from_str(uuid).map_err(|err| Error::UuidFrom(err.to_string()))?;

    let new_name: Option<String> = if new_name.trim().len() == 0 {
        None
    } else {
        Some(new_name.to_string())
    };
    let new_path: Option<String> = if new_path.trim().len() == 0 {
        None
    } else {
        Some(new_path.to_string())
    };

    let file = File::open(FILENAME).map_err(|err| Error::FileOpen(err.to_string()))?;
    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let file = File::create(FILENAME).map_err(|err| Error::FileOpen(err.to_string()))?;

    if let Some(ref mut data) = config.data {
        data.iter_mut()
            .find(|el| el.uuid() == uuid)
            .expect("[ERROR] Game not found")
            .update(new_name, new_path);
    }

    serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))
}

pub fn execute_id(id: &str, new_name: &str, new_path: &str) -> Result<(), Error> {
    let id = id
        .parse::<usize>()
        .map_err(|err| Error::Parse(err.to_string()))?;
    let file = File::open(FILENAME).map_err(|err| Error::FileOpen(err.to_string()))?;
    let config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    return if let Some(data) = config.data {
        let uuid = data
            .get(id)
            .expect("[ERROR] Incorrect index")
            .uuid()
            .to_string();
        execute(&uuid, new_name, new_path)
    } else {
        return Err(Error::DataGet("[ERROR] Games not added".to_string()));
    };
}

pub fn help() {
    println!("[ERROR] Incorrect arguments.\n[HINT] launcher update 'uuid' ['name'|-n] ['path'|-p]\n[HINT] launcher update -i 'index' ['name'|-n] ['path'|-p]")
}
