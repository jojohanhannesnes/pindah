/*
    Pindah dan Simpan
    Pindah untuk move
    Simpan untuk save

    Contoh:
    pindah save {name} -> (coding)
    pindah move coding

*/
use clap::{Parser, ValueEnum};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    Save,
    Move,
}
#[derive(Parser)]
struct Pindah {
    /// available command is save or move
    #[arg(value_enum)]
    command: Command,
    attribute: String,
}

fn main() {
    let home = find_home_directory();
    let config_file = generate_config(&home);
    let pindah = Pindah::parse();
    match pindah.command {
        Command::Save => save(&pindah.attribute, &config_file),
        Command::Move => move_to(&pindah.attribute),
    }
}

fn save(directory_name: &str, mut config: &File) {
    let current_dir = env::current_dir().unwrap();
    let configuration = format!("{}={}", directory_name, current_dir.to_str().unwrap());
    match writeln!(config, "{}", configuration) {
        Ok(_) => {
            println!("Success saving {}", configuration);
        }
        Err(e) => {
            eprintln!("Cannot write to file {}", e);
        }
    }
}

fn move_to(_directory_name: &str) {
    env::set_current_dir("/").unwrap();
}

fn generate_config(config_path: &PathBuf) -> File {
    let is_config_exists = Path::new(&config_path).exists();
    if !is_config_exists {
        File::create(config_path).expect("Error creating .config file")
    } else {
        OpenOptions::new()
            .append(true)
            .read(true)
            .open(config_path)
            .expect("Error opening config file")
    }
}

fn find_home_directory() -> PathBuf {
    let mut home = PathBuf::from(env::var("HOME").expect("cannot read home dir"));
    home.push(".pindah");
    home
}
