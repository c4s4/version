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
    let versions: Vec<String > = software_versions(&args.software);
    // FIXME
    println!("{:?}", versions);
}

fn software_versions(software: &String) -> Vec<String> {
    let app_dir = match env::var("APP_DIR") {
        Ok(val) => val,
        Err(_) => APP_DIR.to_string(),
    };
    let path = format!("{}/{}", app_dir, software);
    let line = run(&"ls".to_string(), &vec![path]);
    let mut versions: Vec<String> = line.split_whitespace().map(str::to_string).collect();
    versions.retain(|v| v != "current");
    versions
}

fn run(command: &String, args: &Vec<String>) -> String {
    let output = Command::new(command).args(args).output();
    if output.is_err() {
        eprintln!("ERROR running command {}: {}", command, output.err().unwrap());
        process::exit(1);
    };
    String::from_utf8(output.unwrap().stdout).unwrap()
}
