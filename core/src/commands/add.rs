use super::{file_open, GAME_DATA};

use core::{print_hint, print_success, Config, Data, Error};
use serde_yaml;

use std::fs::File;

pub fn execute(name: &str, path: &str) -> Result<(), Error> {
    let file = file_open()?;

    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let config = Config {
        data: if let Some(data) = &mut config.data {
            data.push(Data::new(name, path));
            Some(data.to_vec())
        } else {
            Some(vec![Data::new(name, path)])
        },
    };
    let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
    serde_yaml::to_writer(file, &config).map_err(|err| Error::ToWriter(err.to_string()))?;
    let uuid = config.data.clone().unwrap().last().unwrap().uuid();
    let id = config
        .data
        .unwrap()
        .iter()
        .position(|el| el.uuid() == uuid)
        .unwrap()
        .to_string();
    Ok(print_success(
        format!("Game '{name}' added with id {id} at '{path}' ({uuid})",).as_str(),
    ))
}

pub fn help() {
    print_hint("launcher add 'name' 'path'");
}
