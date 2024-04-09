use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;
use colored::Colorize;

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
    // catch errors
    match run(args) {
        Ok(version) => {
            // print selected version
            if version == CURRENT_VERSION {
                println!("{} selected version: System", "OK".green().bold());
            } else {
                println!("{} selected version: {}", "OK".green().bold(), version);
            }
        },
        Err(e) => {
            eprintln!("{} {:#}", "ERROR".red().bold(), e);
            std::process::exit(1);
        },
    }
}

fn run(args: Cli) -> Result<String> {
    // get app directory
    let app_dir = match env::var(APP_DIR_VAR) {
        Ok(val) => val,
        Err(_) => APP_DIR.to_string(),
    };
    // get software versions
    let versions: Vec<String> = software_versions(&app_dir, &args.software)
        .with_context(|| format!("getting software versions for {}", &args.software))?;
    // get selected version
    let selected = selected_version(&app_dir, &args.software)
        .with_context(|| format!("getting selected version for {}", &args.software))?;
    // get version from menu
    let version = version_menu(&versions, &selected)
        .with_context(|| format!("getting version from menu for {}", &args.software))?;
    // select software version
    select_version(&app_dir, &args.software, &version)
        .with_context(|| format!("selecting version {} for {}", &version, &args.software))?;
    Ok(version)
}

// get software versions
fn software_versions(app_dir: &str, software: &str) -> Result<Vec<String>> {
    let dir = format!("{}/{}", &app_dir, &software);
    let mut versions: Vec<String> = Vec::new();
    let files = fs::read_dir(&dir).with_context(|| format!("reading directory {}", &dir))?;
    for file in files {
        let path = file.with_context(|| format!("reading file in directory {}", &dir))?;
        let file_type = path
            .file_type()
            .with_context(|| format!("getting file type for {}", path.path().display()))?;
        if file_type.is_dir() {
            let file_name = path.file_name();
            let version = file_name.to_str()
                .with_context(|| format!("getting file name for {:?}", path))?;
            if version != CURRENT_VERSION {
                versions.push(version.to_string());
            }
        }
    }
    versions.sort();
    Ok(versions)
}

// get version from menu
fn version_menu(versions: &Vec<String>, selected: &str) -> Result<String> {
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
    _ = std::io::stdin()
        .read_line(&mut input)
        .context("reading user input")?;
    let index: usize = input
        .trim()
        .parse()
        .with_context(|| format!("parsing input {}", input.trim()))?;
    if index > versions.len() {
        anyhow::bail!("invalid version index: {}", index);
    }
    // make link to appropriate version
    let version = if index == SYSTEM_RANK {
        SYSTEM_VERSION.to_string()
    } else {
        versions[index - 1].to_string()
    };
    Ok(version)
}

// get selected version
fn selected_version(app_dir: &str, software: &str) -> Result<String> {
    // get current version if set
    let path = format!("{}/{}/{}", app_dir, software, CURRENT_VERSION);
    if Path::new(&path).exists() {
        if let Ok(file) = fs::read_link(&path) {
            let file_name = file
                .file_name()
                .with_context(|| format!("getting file name for {:?}", file))?;
            return Ok(file_name.to_str().unwrap().to_string());
        }
    };
    Ok("".to_string())
}

// select software version making symbolic link
fn select_version(app_dir: &str, software: &str, version: &str) -> Result<()> {
    // go to app directory
    let path = format!("{}/{}", app_dir, software);
    env::set_current_dir(&path).with_context(|| format!("changing to directory {}", path))?;
    // selected system version
    if version == SYSTEM_VERSION {
        // remove symbolic link if it exists
        if Path::new(CURRENT_VERSION).exists() {
            fs::remove_file(CURRENT_VERSION)
                .with_context(|| format!("removing file {}", CURRENT_VERSION))?;
        };
        Ok(())
    } else {
        // selected installed version
        // remove symbolic link if it exists
        if Path::new(CURRENT_VERSION).exists() {
            fs::remove_file(CURRENT_VERSION)
                .with_context(|| format!("removing file {}", CURRENT_VERSION))?;
        }
        // set symbolic link
        symlink(&version, CURRENT_VERSION).with_context(|| {
            format!("creating symbolic link {} -> {}", version, CURRENT_VERSION)
        })?;
        Ok(())
    }
}
