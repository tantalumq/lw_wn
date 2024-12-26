mod commands;

use std::env;

use commands::{add, help, remove, run, update};

// TODO: Add more print

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let args: Vec<&str> = args.iter().map(|x| x.as_str()).collect();
    match args.as_slice() {
        // Add new game
        &["add", name, path] => add::execute(name, path).unwrap(),
        &["add", ..] => add::help(),
        // Run game by id
        &["run", uuid] => run::execute(uuid).unwrap(),
        &["run", "-i", id] => run::execute_id(id).unwrap(),
        &["run", ..] => run::help(),
        // Update game data
        &["update", uuid, "-n", path] => update::execute(uuid, "", path).unwrap(),
        &["update", uuid, name, "-p"] => update::execute(uuid, name, "").unwrap(),
        &["update", uuid, name, path] => update::execute(uuid, name, path).unwrap(),

        &["update", "-i", id, "-n", path] => update::execute_id(id, "", path).unwrap(),
        &["update", "-i", id, name, "-p"] => update::execute_id(id, name, "").unwrap(),
        &["update", "-i", id, name, path] => update::execute_id(id, name, path).unwrap(),
        &["update", ..] => update::help(),
        // Remove game
        &["remove", uuid] => remove::execute(uuid).unwrap(),
        &["remove", "-i", id] => remove::execute_id(id).unwrap(),
        &["remove", ..] => remove::help(),
        _ => help(),
    };
}
