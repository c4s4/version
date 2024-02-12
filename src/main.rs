use clap::Parser;
use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use std::process;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DIR: &str = "/opt";
const APP_DIR_VAR: &str = "APP_DIR";
const CURRENT_VERSION: &str = "current";
const SYSTEM_VERSION: &str = "system";
const SYSTEM_RANK: usize = 0;

/// Select software version from menu
#[derive(Parser)]
struct Cli {
    /// The version
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
        error("Software not set");
    }
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
        println!("Selected version: system");
    } else {
        println!("Selected version: {}", version);
    }
}

// get software versions
fn software_versions(app_dir: &str, software: &str) -> Vec<String> {
    let dir = format!("{}/{}", &app_dir, &software);
    let mut versions: Vec<String> = Vec::new();
    let result = fs::read_dir(&dir);
    if result.is_err() {
        error(&format!("reading directory {}", dir));
    }
    for file in result.unwrap() {
        if !file.is_err() {
            let path = file.unwrap().path();
            if path.is_dir() {
                let name = path.file_name().unwrap().to_str().unwrap();
                if name != CURRENT_VERSION {
                    versions.push(name.to_string());
                }
            }
        }
    }
    versions
}

// get version from menu
fn version_menu(versions: &Vec<String>, selected: &str) -> String {
    // print versions menu
    println!("Please choose a version:");
    println!("0: System");
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
    std::io::stdin().read_line(&mut input).unwrap();
    let index: usize = input.trim().parse().unwrap();
    if index > versions.len() {
        error(&format!("invalid version index: {}", index));
    }
    // make link to appropriate version
    let version = if index == SYSTEM_RANK {
        SYSTEM_VERSION.to_string()
    } else {
        versions[index - 1].to_string()
    };
    version
}

fn selected_version(app_dir: &str, software: &str) -> String {
    // get current version if set
    let path = format!("{}/{}/{}", app_dir, software, CURRENT_VERSION);
    if Path::new(&path).exists() {
        let result = fs::read_link(&path);
        if result.is_ok() {
            return result.unwrap().file_name().unwrap().to_str().unwrap().to_string();
        };
    };
    "".to_string()
}

// select software version making symbolic link
fn select_version(app_dir: &str, software: &str, version: &str) {
    // go to app directory
    let path = format!("{}/{}", app_dir, software);
    if !env::set_current_dir(&path).is_ok() {
        error(&format!("changing to directory {}", path));
    }
    if version == SYSTEM_VERSION {
        if Path::new(CURRENT_VERSION).exists() {
            let result = std::fs::remove_file(CURRENT_VERSION);
            if !result.is_ok() {
                error(&format!(
                    "removing file '{}': {:?}",
                    CURRENT_VERSION,
                    result.err()
                ));
            }
        }
    } else {
        // remove symbolic link if it exists
        if Path::new(CURRENT_VERSION).exists() {
            let result = std::fs::remove_file(CURRENT_VERSION);
            if !result.is_ok() {
                error(&format!("removing file '{}': {:?}", version, result.err()));
            }
        }
        // set symbolic link
        let result = symlink(&version, CURRENT_VERSION);
        if !result.is_ok() {
            error(&format!(
                "creating symbolic link {} -> {}: {:?}",
                version,
                CURRENT_VERSION,
                result.err()
            ));
        }
    }
}

// print error message and exit
fn error(message: &str) {
    eprintln!("ERROR {}", message);
    process::exit(1);
}
