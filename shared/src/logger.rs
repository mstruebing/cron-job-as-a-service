// stdlib
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

// modules
use dotenv::dotenv;

// internal
// TODO: Why do I need a `crate` here?
use crate::utils::get_current_timestamp;

pub fn info(msg: &str) {
    println!("INFO: {}", msg)
}

pub fn log(msg: &str) {
    let msg: &str = &format!("{}: {}", get_current_timestamp(), msg);
    println!("{}", msg);
    write_to_log_file(msg)
}

pub fn debug(msg: &str) {
    match dotenv() {
        Ok(_) => {
            if env::var("DEBUG").is_ok() {
                println!("DEBUG: {}", msg)
            }
        }
        Err(err) => log(&format!("ERROR: {:?}", err)),
    }
}

pub fn write_to_log_file(msg: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        // TODO: Make this configurable somehow
        .open("log.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", msg) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
