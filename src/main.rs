use clap::Parser;
use std::env;
use std::process;
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_DIR: &str = "/opt";

/// Run command ensuring only one instance is running on this system
#[derive(Parser)]
struct Cli {
    /// The lone version
    #[arg(short, long)]
    version: bool,
    /// Software to set version for
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
    // get software versions
    let versions: Vec<str> = software_versions(args.software);
}

fn software_versions(software: String) -> &Vec<str> {
    let app_dir = match env::var("APP_DIR") {
        Ok(val) => val,
        Err(e) => APP_DIR.to_string(),
    };
    let path = format!("{}/{}", app_dir, software);
    let output = Command::new("ls")
        .arg(path)
        .output()
        .expect("failed to execute process");
    let versions: Vec<str> = String::from_utf8(output.stdout).unwrap().split("\n").collect();
    versions.retain(|&v| v != "current");
    &versions
}

fn run(command: String) {
    if let Err(err) = Command::new(command).status() {
        eprintln!("ERROR running command: {err}");
        process::exit(1);
    };
}
