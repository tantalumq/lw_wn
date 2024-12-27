use std::{
    fmt::Display,
    fs::{remove_file, File},
    path::Path,
    process::exit,
};

use colored::Colorize;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    uuid: Uuid,
    name: String,
    path: String,
    time: u32,
}

impl Data {
    pub fn new(name: &str, path: &str) -> Data {
        let uuid = Uuid::new_v4();
        let name = name.into();
        let path = path.into();

        Data {
            uuid,
            name,
            path,
            time: 0u32,
        }
    }
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn time(&self) -> u32 {
        self.time
    }
    pub fn update(
        &mut self,
        new_name: Option<String>,
        new_path: Option<String>,
    ) -> (String, String) {
        let (old_name, old_path) = (self.name(), self.path());
        if let Some(name) = new_name {
            self.name = name;
        }
        if let Some(path) = new_path {
            self.path = path;
        }
        (old_name, old_path)
    }
    pub fn add_time(&mut self, time: u32) {
        self.time += time;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub data: Option<Vec<Data>>,
}

#[derive(Debug)]
pub enum Error {
    FileOpen(String),
    FileRead(String),
    FileWrite(String),
    FromReader(String),
    ToWriter(String),
    CommandRun(String),
    UuidFrom(String),
    DataGet(String),
    PathFrom(String),
    FileNotExe,
    Parse(String),
    Metadata(String),
    Child(String),
    Index,
    GameIsNotExists,
    PathIsNotExists,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileOpen(msg) => write!(f, "{}", msg.red()),
            Error::FileRead(msg) => write!(f, "{}", msg.red()),
            Error::FileWrite(msg) => write!(f, "{}", msg.red()),
            Error::FromReader(msg) => write!(f, "{}", msg.red()),
            Error::ToWriter(msg) => write!(f, "{}", msg.red()),
            Error::CommandRun(msg) => write!(f, "{}", msg.red()),
            Error::UuidFrom(msg) => write!(f, "{}", msg.red()),
            Error::DataGet(msg) => write!(f, "{}", msg.red()),
            Error::PathFrom(msg) => write!(f, "{}", msg.red()),
            Error::FileNotExe => write!(f, "{}", "File is not exe".red()),
            Error::Parse(msg) => write!(f, "{}", msg.red()),
            Error::Metadata(msg) => write!(f, "{}", msg.red()),
            Error::Child(msg) => write!(f, "{}", msg.red()),
            Error::Index => write!(f, "{}", "Incorrect index".red()),
            Error::GameIsNotExists => write!(f, "{}", "Game is not exists".red()),
            Error::PathIsNotExists => write!(f, "{}", "Path is not exsits".red()),
        }
    }
}

const LOCK: &'static str = "./lock";

pub fn lock() {
    if !Path::new("./lock").exists() {
        File::create(LOCK).expect("[ERROR] Can`t create lock file");
    } else {
        exit(0);
    }
}

pub fn unlock() {
    remove_file(LOCK).expect("[ERROR] Can`t delete lock file");
}

pub fn print_error<T>(msg: T)
where
    T: Display,
{
    println!("{} {}", "[ERROR]".red().bold(), msg)
}

pub fn print_success<T>(msg: T)
where
    T: Colorize,
{
    println!("{} {}", "[SUCCESS]".green().bold(), msg.green())
}
