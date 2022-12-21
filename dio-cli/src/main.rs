//! # Usage
//!
//! ```bash
//! $ cargo install --path .
//! $ dio --option facts --key 12
//! fact 12: Lorem ipsum dolor sit amet, consectetur
//! ```

use clap::Parser;
use dio_cli::{DioFacts, DioPrinciples, StoreCount};
use dotenv::dotenv;
use std::fs::File;

// #[tokio::main]
fn main() {
    dotenv().ok(); // cronjob::CronJob::run_cron();
    main_cli(); // hashfile::encrypt_decrypt();
}

/// Simple program to display one of each options.
#[derive(Parser, Debug)]
#[command(author,version,about,long_about = None)]
pub(crate) struct Args {
    /// Request option for either principles or facts.
    #[arg(short, long)]
    pub(crate) option: String,

    /// Key number of Principles or Facts to display.
    #[arg(short, long, default_value_t = 1)]
    pub(crate) key: u8,
}
fn main_cli() {
    let args = Args::parse();
    match args.option.as_str() {
        "principles" => Dio::handle_principles(args),
        "facts" => Dio::handle_facts(args),
        _ => println!("Invalid option. Please use either 'principles' or 'facts'"),
    }
}

#[derive(Default, Debug)]
pub struct Dio;
impl Dio {
    /// .
    fn handle_facts(args: Args) {
        if !(0u8 < args.key && args.key <= (StoreCount::Facts as u8)) {
            eprintln!("Index out of bounds");
            std::process::exit(1);
        }
        let facts = Self::read_file_facts();
        let fact: &String = &facts[args.key as usize - 1];
        println!("{}", fact);
    }

    /// .
    fn handle_principles(args: Args) {
        if !(0u8 < args.key && args.key <= (StoreCount::Principles as u8)) {
            eprintln!("Index out of bounds");
            std::process::exit(1);
        }
        let principles = Self::read_file_principles();
        let principle: &String = &principles[args.key as usize - 1];
        println!("{}", principle);
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    fn read_file_facts() -> Vec<String> {
        let rdr: File = match File::open::<&str>("data.json") {
            Ok(t) => t,
            Err(_) => {
                println!("Failed to open file");
                std::process::exit(1);
            }
        };
        match serde_json::from_reader::<_, DioFacts>(&rdr) {
            Ok(contents) => contents,
            _ => panic!("Could not read file"),
        }
        .facts
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    fn read_file_principles() -> Vec<String> {
        let rdr: File = match File::open::<&str>("data.json") {
            Ok(t) => t,
            Err(_) => {
                println!("Failed to read file");
                std::process::exit(1);
            }
        };
        match serde_json::from_reader::<_, DioPrinciples>(&rdr) {
            Ok(contents) => contents,
            _ => panic!("Could not read file"),
        }
        .principles
    }
}
