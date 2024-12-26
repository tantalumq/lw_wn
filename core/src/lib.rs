use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    uuid: Uuid,
    name: String,
    path: String,
    time: f32,
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
            time: 0f32,
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
    pub fn update(&mut self, new_name: Option<String>, new_path: Option<String>) {
        if let Some(name) = new_name {
            self.name = name;
        }
        if let Some(path) = new_path {
            self.path = path;
        }
    }
    pub fn add_time(&mut self, time: f32) {
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
}

impl Error {}
