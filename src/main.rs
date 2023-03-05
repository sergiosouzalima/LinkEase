/*
Program name....: main.rs
Author..........: Sergio Lima
Date created....: 2023/03/04

How to compile:
    Windows:
        cargo build --target=x86_64-pc-windows-gnu --release
    Linux
        cargo build --release
*/

use indoc::indoc;
use regex::Regex;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::env;
use toml;

const APP_NAME: &str = "LinkEase";
const APP_BRIEFCASE_EMOJI: char = '\u{1F4BC}';
const APP_CHECKMARK_EMOJI: char = '\u{2705}';
const VERSION: &str = "0.2.0";
const AUTHOR: &str = "Sergio Lima <sergiosouzalima@gmail.com>";
const DESC: &str = "LinkedIn Connect Manager CLI Program.";
const ABOUT: &str = "A Rust CLI for managing LinkedIn connections with ease.\n \
This program processes LinkedIn connections data (Connections.csv file)\n \
and creates a file named result_connections.csv with filtered connections.\n \
The file config.toml stores all the filtering criteria that this app needs.\n\n \
If you need any assistance or have any suggestions for improvements,\n \
please don't hesitate to reach out to us at sergiosouzalima@gmail.com.";

#[derive(Deserialize)]
struct Data {
    config: Config,
}

#[derive(Deserialize)]
struct Config {
    source_file: String,
    target_file: String,
    source_file_skip_rows: u16,
    email_validation: bool,
    position_filter_for: Vec<String>,
}

const CONFIG_FILE_NAME: &str = "config.toml";
const REGEX: &str = r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+(\.[a-zA-Z0-9-.])?$";

fn is_valid_email(re: &Regex, email: &str) -> bool {
    re.is_match(email)
}

fn check_match(position: &str, elements: &Vec<String>) -> bool {
    for element in elements.iter() {
        if position.to_uppercase().contains(&element.to_uppercase()) {
            return true;
        }
    }
    false
}

fn deserialize_toml(file_contents: &str) -> Result<Data, Box<dyn Error>> {
    toml::from_str(file_contents).map_err(|e| e.into())
}

fn process_command(command: &str) {
    match command {
        "--version" | "-v" => println!("Version {}", VERSION),
        "--help" | "-h" => println!("Usage: linkease \
                                     [--version | -v | --help | -h | --about | -a]"),
        "--about" | "-a" => println!("About {}:\n {}", APP_NAME, ABOUT),
        _ => println!("Unknown command: {}", command),
    }
    print!("\n");
}


fn main() {
    println!("\n{} {} - {}\nAuthor: {}\n",APP_BRIEFCASE_EMOJI, APP_NAME,DESC,AUTHOR);

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let command = &args[1];
        process_command(command);
        return;
    }

    if !Path::new(CONFIG_FILE_NAME).exists() {
        let content = indoc! {r#"
            [config]
            source_file = "Connections.csv"
            target_file = "result_connections.csv"
            source_file_skip_rows = 3
            email_validation = true
            position_filter_for = ["App", "Application", "Back End", "Desenvolvedor",
            "Developer", "Engineer", "Front End", "Full Stack", "Mobile", "Programador",
            "Programmer", "Soft", "Software", "Sistemas", "Systems", "Web"]
        "#};
        println!("Creating config file {}....\n", CONFIG_FILE_NAME);
        fs::write(CONFIG_FILE_NAME, content).expect("Failed to create config file.");
    }
    let contents = fs::read_to_string(CONFIG_FILE_NAME).expect("Failed to open config file");

    let data = match deserialize_toml(&contents) {
        Ok(d) => d,
        Err(e) => {
            println!("Error deserializing {}: {}\n", CONFIG_FILE_NAME, e);
            return;
        }
    };

    // Check if Connections.csv file exists.
    if !Path::new(&data.config.source_file).exists() {
        println!("File {} not found!\n", data.config.source_file);
        return;
    }
    let file = File::open(data.config.source_file).expect("Failed to open file.");

    // Read Connections.csv file,
    // filter for valid connections,
    // and create new file with "valid LinkedIn connections" in it.
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    // Skip lines
    for _ in 0..data.config.source_file_skip_rows {
        lines.next();
    }

    // Get the header
    let header = lines.next().unwrap().unwrap();

    let re = Regex::new(REGEX).unwrap();
    let mut valid_email_count = 0;
    let mut row_count = 0;
    let mut valid_rows = vec![header];
    // Read the remaining lines
    for line in lines {
        let row = line.unwrap();
        let values: Vec<&str> = row.split(',').collect();

        let email = values[2];
        let position = values[4];
        if check_match(&position, &data.config.position_filter_for) {
            let mut valid_email: bool = true;
            if data.config.email_validation {
                valid_email = is_valid_email(&re, email);
            }
            if valid_email {
                valid_email_count += 1;
                valid_rows.push(row);
            }
        }
        row_count += 1;
    }

    println!("Connections.................: {}", row_count);
    println!("Connections with valid email: {}", valid_email_count);
    println!("New file created............: {}", data.config.target_file);

    let mut new_file = File::create(data.config.target_file).expect("Failed to create new file.");
    for row in valid_rows {
        writeln!(new_file, "{}", row.replace(",", ";")).expect("Failed to write to new file.");
    }

    println!("\nProgram execution finished.{}\n", APP_CHECKMARK_EMOJI)
}
