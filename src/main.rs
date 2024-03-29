//! Quake 3 log parser

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

/// Module responsible for the CLI
/// Both the CLI configuration and argument parsing
/// as well as printing the reports
mod cli;
/// Module responsible for the data representation from the log
/// like the means of death and the players data
/// the `PlayerData` struct and the `MeanDeath` enum
mod quake3_data;
/// Module responsible for the parsing functionalities
mod quake3_parser;

use cli::{term_report, Cli, ReportFormat};
use quake3_parser::parser::{scan_file, Game};

use clap::Parser;
use std::fs;

#[cfg_attr(coverage_nightly, coverage(off))]
/// main function
fn main() {
    let cli = Cli::parse();

    let filepath = &cli.log_file;
    let log_str = fs::read_to_string(filepath).expect("Error reading file");

    let games: Vec<Game> = match scan_file(&log_str) {
        Ok(games) => games,
        Err(err) => {
            eprintln!("Error parsing file {filepath:?}: {err}");
            return;
        }
    };

    match cli.report_format {
        ReportFormat::Html => {}
        ReportFormat::Text => {
            let result = term_report(&games, &cli.report_type);
            match result {
                Ok(term_table) => println!("{term_table}"),
                Err(err) => eprintln!("{err}"),
            }
        }
    }
}
