/*
    Pindah dan Simpan
    Pindah untuk move
    Simpan untuk save

    Contoh:
    pindah save {name} -> (coding)
    pindah move coding

*/
use clap::{Parser, ValueEnum};
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, process};
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Instruction {
    Save,
    Move,
}
#[derive(Parser)]
struct Pindah {
    /// available Instruction is save or move
    #[arg(value_enum)]
    instruction: Instruction,
    attribute: String,
}

fn main() -> anyhow::Result<()> {
    let home = find_home_directory();
    let config_file = generate_config(&home);
    let pindah = Pindah::parse();
    match pindah.instruction {
        Instruction::Save => save(&pindah.attribute, &config_file),
        Instruction::Move => move_to(&pindah.attribute, &home),
    };
    Ok(())
}

fn save(directory_alias: &str, mut config: &File) {
    let current_dir = env::current_dir().unwrap();
    let configuration = format!("{}={}", directory_alias, current_dir.to_str().unwrap());
    match writeln!(config, "{}", configuration) {
        Ok(_) => {
            println!("Success saving {}", configuration);
        }
        Err(e) => {
            eprintln!("Cannot write to file {}", e);
        }
    }
}

fn move_to(directory_name_alias: &str, config: &PathBuf) {
    let data = read_to_string(config).unwrap();
    for line in data.split('\n') {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if directory_name_alias == parts[0] {
            println!("{}", parts[1]);
            process::exit(1);
        }
    }
    process::exit(99);
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
