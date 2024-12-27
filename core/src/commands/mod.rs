use core::{Config, Error};
use std::{
    fmt::format,
    fs::{metadata, File, OpenOptions},
    io::Write,
};

pub mod add;
pub mod list;
pub mod remove;
pub mod run;
pub mod update;

pub const GAME_DATA: &'static str = "./games.yml";

pub fn help() {}

pub fn file_open() -> Result<File, Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(GAME_DATA)
        .map_err(|err| Error::FileOpen(err.to_string()))?;

    if metadata(GAME_DATA)
        .map_err(|err| Error::Metadata(err.to_string()))?
        .len()
        == 0
    {
        file.write_all("data: Null".as_bytes())
            .map_err(|err| Error::FileWrite(err.to_string()))?;
    }
    Ok(file)
}

pub fn parse_time(time: u32) -> String {
    let mut secundes = time;
    let mut minutes = 0u32;
    while secundes >= 60 {
        secundes -= 60;
        minutes += 1;
    }

    let mut hours = 0u32;
    while minutes >= 60 {
        minutes -= 60;
        hours += 1;
    }

    let mut days = 0u32;
    while hours >= 24 {
        hours -= 24;
        days += 1;
    }
    let mut buf = format!("{}s", secundes);
    if minutes > 0 {
        buf.insert_str(0, &format!("{}m", minutes));
    }
    if hours > 0 {
        buf.insert_str(0, &format!("{}h", hours));
    }
    if days > 0 {
        buf.insert_str(0, &format!("{}d", days));
    }
    buf
}
