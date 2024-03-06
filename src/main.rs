use clap::Parser;
use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process;

const APP_DIR: &str = "/opt";
const APP_DIR_VAR: &str = "APP_DIR";
const CURRENT_VERSION: &str = "current";
const SYSTEM_VERSION: &str = "system";
const SYSTEM_RANK: usize = 0;

/// Select software version from menu
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Software to set version for
    software: String,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // get app directory
    let app_dir = match env::var(APP_DIR_VAR) {
        Ok(val) => val,
        Err(_) => APP_DIR.to_string(),
    };
    // get software versions
    let versions: Vec<String> = software_versions(&app_dir, &args.software);
    // get selected version
    let selected = selected_version(&app_dir, &args.software);
    // get version from menu
    let version = version_menu(&versions, &selected);
    // select software version
    select_version(&app_dir, &args.software, &version);
    // print selected version
    if version == CURRENT_VERSION {
        println!("Selected version: System");
    } else {
        println!("Selected version: {}", version);
    }
}

// get software versions
fn software_versions(app_dir: &str, software: &str) -> Vec<String> {
    let dir = format!("{}/{}", &app_dir, &software);
    let mut versions: Vec<String> = Vec::new();
    let files = match fs::read_dir(&dir) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("ERROR reading directory {}: {}", dir, err);
            process::exit(1);
        }
    };
    for file in files {
        let path = match file {
            Ok(file) => file.path(),
            Err(err) => {
                eprintln!("ERROR reading directory {}: {}", dir, err);
                process::exit(1);
            }
        };
        if path.is_dir() {
            let version = path.file_name().unwrap().to_str().unwrap();
            if version != CURRENT_VERSION {
                versions.push(version.to_string());
            }
        }
    }
    versions.sort();
    versions
}

// get version from menu
fn version_menu(versions: &Vec<String>, selected: &str) -> String {
    // print versions menu
    println!("Please choose a version:");
    if selected == "" {
        println!("0: System *");
    } else {
        println!("0: System");
    }
    let mut index = 1;
    for version in versions {
        if version == selected {
            println!("{index}: {version} *");
        } else {
            println!("{index}: {version}");
        }
        index += 1;
    }
    // get user input
    let mut input = String::new();
    if let Err(err) = std::io::stdin().read_line(&mut input) {
        eprintln!("ERROR reading input: {}", err);
        process::exit(1);
    }
    let index: usize = match input.trim().parse() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("ERROR parsing input: {}", err);
            process::exit(1);
        }
    };
    if index > versions.len() {
        eprintln!("ERROR invalid version index: {}", index);
        process::exit(1);
    }
    // make link to appropriate version
    let version = if index == SYSTEM_RANK {
        SYSTEM_VERSION.to_string()
    } else {
        versions[index - 1].to_string()
    };
    version
}

// get selected version
fn selected_version(app_dir: &str, software: &str) -> String {
    // get current version if set
    let path = format!("{}/{}/{}", app_dir, software, CURRENT_VERSION);
    if Path::new(&path).exists() {
        if let Ok(file) = fs::read_link(&path) {
            return file.file_name().unwrap().to_str().unwrap().to_string();
        }
    };
    "".to_string()
}

// select software version making symbolic link
fn select_version(app_dir: &str, software: &str, version: &str) {
    // go to app directory
    let path = format!("{}/{}", app_dir, software);
    if let Err(err) = env::set_current_dir(&path) {
        eprintln!("ERROR changing to directory {}: {}", path, err);
        process::exit(1);
    }
    // selected system version
    if version == SYSTEM_VERSION {
        // remove symbolic link if it exists
        if Path::new(CURRENT_VERSION).exists() {
            if let Err(err) = fs::remove_file(CURRENT_VERSION) {
                eprintln!("ERROR removing file {}: {}", CURRENT_VERSION, err);
                process::exit(1);
            }
        }
    } else {
        // selected installed version
        // remove symbolic link if it exists
        if Path::new(CURRENT_VERSION).exists() {
            if let Err(err) = fs::remove_file(CURRENT_VERSION) {
                eprintln!("ERROR removing file {}: {}", CURRENT_VERSION, err);
                process::exit(1);
            }
        }
        // set symbolic link
        if let Err(err) = symlink(&version, CURRENT_VERSION) {
            eprintln!(
                "ERROR creating symbolic link '{}' -> '{}': {}",
                version, CURRENT_VERSION, err
            );
            process::exit(1);
        }
    }
}
