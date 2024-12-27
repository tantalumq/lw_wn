mod commands;

use core::{lock, unlock};
use std::env;

use commands::{add, help, list, remove, run, update};

// TODO: Add more print
// TODO: No panics

fn main() {
    lock();
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let args: Vec<&str> = args.iter().map(|x| x.as_str()).collect();
    match args.as_slice() {
        // Show all games
        &["list"] => list::execute().unwrap(),
        // Add new game
        &["add", name, path] => add::execute(name, path).unwrap(),
        &["add", ..] => add::help(),
        // Run game
        &["run", "-uuid", uuid] => run::execute_uuid(uuid).unwrap(),
        &["run", id] => run::execute(id).unwrap(),
        &["run", ..] => run::help(),
        // Update game data
        &["update", "--uuid", uuid, "-n", path] => update::execute_uuid(uuid, "", path).unwrap(),
        &["update", "--uuid", uuid, name, "-p"] => update::execute_uuid(uuid, name, "").unwrap(),
        &["update", "--uuid", uuid, name, path] => update::execute_uuid(uuid, name, path).unwrap(),

        &["update", id, "-n", path] => update::execute(id, "", path).unwrap(),
        &["update", id, name, "-p"] => update::execute(id, name, "").unwrap(),
        &["update", id, name, path] => update::execute(id, name, path).unwrap(),
        &["update", ..] => update::help(),
        // Remove game
        &["remove", "--uuid", uuid] => remove::execute_uuid(uuid).unwrap(),
        &["remove", id] => remove::execute(id).unwrap(),
        &["remove", ..] => remove::help(),
        _ => help(),
    };

    unlock();
}
