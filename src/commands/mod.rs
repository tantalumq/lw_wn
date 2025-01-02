use core::{print_error, print_hint, Error};
use std::{
    fs::{metadata, File, OpenOptions},
    io::{stdin, stdout, Write},
};

use colored::Colorize;
use shellwords::split;

mod add;
mod list;
mod remove;
mod run;
mod update;

pub const GAME_DATA: &'static str = "./games.yml";

pub fn help() {
    add::help();
    run::help();
    update::help();
    remove::help();
    print_hint("print 'exit' to break loop");
}

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
        buf.insert_str(0, &format!("{}m ", minutes));
    }
    if hours > 0 {
        buf.insert_str(0, &format!("{}h ", hours));
    }
    if days > 0 {
        buf.insert_str(0, &format!("{}d ", days));
    }
    buf
}

pub fn execute(args: &mut Vec<String>, is_loop: bool) -> Result<(), Error> {
    let args: Vec<&str> = args.iter().map(|x| x.as_str()).collect();

    match args.as_slice() {
        // Start loop
        &[] | &["core"] => {
            if !is_loop {
                execute_loop()
            } else {
                Ok(())
            }
        }
        // Show all games
        &["list"] | &["core", "list"] => list::execute(),
        // Add new game
        &["add", name, path] | &["core", "add", name, path] => add::execute(name, path),
        &["add", ..] | ["core", "add", ..] => Ok(add::help()),
        // Run game
        &["run", "-u", uuid] | &["core", "run", "-u", uuid] => run::execute_uuid(uuid),
        &["run", id] | &["core", "run", id] => run::execute(id),
        &["run", ..] | &["core", "run", ..] => Ok(run::help()),
        // Update game data
        &["update", "-u", uuid, "-n", path] | &["core", "update", "-u", uuid, "-n", path] => {
            update::execute_uuid(uuid, "", path)
        }
        &["update", "-u", uuid, name, "-p"] | &["core", "update", "-u", uuid, name, "-p"] => {
            update::execute_uuid(uuid, name, "")
        }
        &["update", "-u", uuid, name, path] | &["core", "update", "-u", uuid, name, path] => {
            update::execute_uuid(uuid, name, path)
        }
        &["update", id, "-n", path] | &["core", "update", id, "-n", path] => {
            update::execute(id, "", path)
        }
        &["update", id, name, "-p"] | &["core", "update", id, name, "-p"] => {
            update::execute(id, name, "")
        }
        &["update", id, name, path] | &["core", "update", id, name, path] => {
            update::execute(id, name, path)
        }
        &["update", ..] | &["core", "update", ..] => Ok(update::help()),
        // Remove game
        &["remove", "-u", uuid] | &["core", "remove", "-u", uuid] => remove::execute_uuid(uuid),
        &["remove", id] | &["core", "remove", id] => remove::execute(id),
        &["remove", ..] | &["core", "remove", ..] => Ok(remove::help()),
        _ => Ok(help()),
    }
}

pub fn execute_loop() -> Result<(), Error> {
    let mut args: Vec<String> = parse_input(read_input()?)?;
    Ok(loop {
        match args
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .as_slice()
        {
            &["exit"] | &["core", "exit"] => break,
            _ => {
                let output = execute(&mut args.iter().map(|x| x.to_string()).collect(), true);

                if let Err(error) = output {
                    print_error(error)
                }

                args = parse_input(read_input()?)?;
            }
        }
    })
}

fn parse_input(input: String) -> Result<Vec<String>, Error> {
    let args: Vec<String> = split(&input)
        .map_err(|err| Error::Input(err.to_string()))?
        .iter()
        .map(|el| match el.to_lowercase().as_str() {
            "core" | "exit" | "list" | "add" | "run" | "update" | "remove" => {
                el.trim().to_lowercase()
            }
            _ => el.trim().to_string(),
        })
        .collect();
    Ok(args)
}

fn read_input() -> Result<String, Error> {
    print!("{} ", ">>".yellow());
    stdout()
        .flush()
        .map_err(|err| Error::Flush(err.to_string()))?;
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .map_err(|err| Error::Input(err.to_string()))?;
    Ok(buf)
}
