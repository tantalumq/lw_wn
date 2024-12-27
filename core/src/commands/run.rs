use core::{Config, Error};
use std::{
    fs::File,
    path::PathBuf,
    process::Command,
    str::FromStr,
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::{Duration, Instant},
    usize,
};

use uuid::Uuid;

use super::{file_open, GAME_DATA};

pub fn execute(id: &str) -> Result<(), Error> {
    let id = id
        .parse::<usize>()
        .map_err(|err| Error::Parse(err.to_string()))?;
    let file = file_open()?;
    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let path: String = if let Some(ref data) = config.data {
        let data = data.get(id).expect("[ERROR] Game not found");
        data.path()
    } else {
        return Err(Error::DataGet("[ERROR] Games not added".to_string()));
    };

    let path = PathBuf::from_str(&path).map_err(|err| Error::PathFrom(err.to_string()))?;
    if let Some(extension) = path.extension() {
        if extension != "exe" {
            return Err(Error::FileNotExe);
        }
    };
    let is_game_running = Arc::new(Mutex::new(true));
    let is_game_running_clone = Arc::clone(&is_game_running);
    thread::spawn(move || {
        let mut child = Command::new(path).spawn().unwrap();
        let _ = child.wait();
        *is_game_running_clone.lock().unwrap() = false;
    });

    while *is_game_running.lock().unwrap() {
        config
            .data
            .as_mut()
            .unwrap()
            .get_mut(id)
            .expect("[ERROR] Game not found")
            .add_time(1);
        sleep(Duration::from_secs(1));
        let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
        serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))?;
    }
    Ok(())
}

pub fn execute_uuid(uuid: &str) -> Result<(), Error> {
    let uuid = Uuid::from_str(uuid).map_err(|err| Error::UuidFrom(err.to_string()))?;
    let file = file_open()?;

    let mut config: Config =
        serde_yaml::from_reader(&file).map_err(|err| Error::FromReader(err.to_string()))?;

    let path: String = if let Some(ref data) = config.data {
        let data = data
            .iter()
            .find(|el| el.uuid() == uuid)
            .expect("[ERROR] Game not found");
        data.path()
    } else {
        return Err(Error::DataGet("[ERROR] Games not added".to_string()));
    };

    let path = PathBuf::from_str(&path).map_err(|err| Error::PathFrom(err.to_string()))?;
    if let Some(extension) = path.extension() {
        if extension != "exe" {
            return Err(Error::FileNotExe);
        }
    };
    let is_game_running = Arc::new(Mutex::new(true));
    let is_game_running_clone = Arc::clone(&is_game_running);
    thread::spawn(move || {
        let mut child = Command::new(path).spawn().unwrap();
        let _ = child.wait();
        *is_game_running_clone.lock().unwrap() = false;
    });

    while *is_game_running.lock().unwrap() {
        config
            .data
            .as_mut()
            .unwrap()
            .iter_mut()
            .find(|el| el.uuid() == uuid)
            .expect("[ERROR] Game not found")
            .add_time(1);
        sleep(Duration::from_secs(1));
        let file = File::create(GAME_DATA).map_err(|err| Error::FileOpen(err.to_string()))?;
        serde_yaml::to_writer(&file, &config).map_err(|err| Error::ToWriter(err.to_string()))?;
    }
    Ok(())
}

pub fn help() {
    println!("[ERROR] Incorrect arguments.\n[HINT] launcher run 'id' or launcher run --uuid 'uuid'")
}
