use clap::Parser;
use std::env;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process;
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DIR: &str = "/opt";
const CURRENT_VERSION: &str = "current";

/// Run command ensuring only one instance is running on this system
#[derive(Parser)]
struct Cli {
    /// The lone version
    #[arg(short, long)]
    version: bool,
    /// Software to set version for
    #[arg(default_value(""))]
    software: String,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // print version and exit
    if args.version {
        println!("{}", VERSION);
        return;
    }
    // check if software is set
    if args.software.is_empty() {
        eprintln!("Software not set");
        process::exit(1);
    }
    // get app directory
    let app_dir = match env::var("APP_DIR") {
        Ok(val) => val,
        Err(_) => APP_DIR.to_string(),
    };
    // get software versions
    let versions: Vec<String> = software_versions(&app_dir, &args.software);
    // print versions menu
    println!("Please choose a version:");
    println!("0: System");
    let mut index = 1;
    for version in &versions {
        println!("{index}: {version}");
        index += 1;
    }
    // get user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let index: usize = input.trim().parse().unwrap();
    if index > versions.len() {
        eprintln!("Invalid version index: {}", index);
        process::exit(1);
    }
    // make link to appropriate version
    let version = if index == 0 {
        CURRENT_VERSION.to_string()
    } else {
        versions[index - 1].to_string()
    };
    // go to app directory
    let path = format!("{}/{}", app_dir, args.software);
    if !env::set_current_dir(&path).is_ok() {
        eprintln!("ERROR changing to directory {}", path);
        process::exit(1);
    }
    if version == CURRENT_VERSION {
        // remove symbolic link
        if Path::new(&version).exists() {
            if !std::fs::remove_file(&version).is_ok() {
                eprintln!("ERROR removing file {}", version);
                process::exit(1);
            }
        }
    } else {
        // set symbolic link
        if !symlink(&version, CURRENT_VERSION).is_ok() {
            eprintln!(
                "ERROR creating symbolic link {} -> {}",
                version, CURRENT_VERSION
            );
            process::exit(1);
        }
    }
}

fn software_versions(app_dir: &String, software: &String) -> Vec<String> {
    let path = format!("{}/{}", app_dir, software);
    let line = run(&"ls".to_string(), &vec![path]);
    let mut versions: Vec<String> = line.split_whitespace().map(str::to_string).collect();
    versions.retain(|v| v != CURRENT_VERSION);
    versions.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
    versions
}

fn run(command: &String, args: &Vec<String>) -> String {
    let output = Command::new(command).args(args).output();
    if output.is_err() {
        eprintln!(
            "ERROR running command {}: {}",
            command,
            output.err().unwrap()
        );
        process::exit(1);
    };
    String::from_utf8(output.unwrap().stdout).unwrap()
}
