//! Quake 3 log parser

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

/// Module responsible for the CLI
/// Both the CLI configuration and argument parsing
mod cli;
/// Module responsible for the data representation from the log
/// like the means of death and the players data
/// the `PlayerData` struct and the `MeanDeath` enum
mod quake3_data;
/// Module responsible for the parsing functionalities
mod quake3_parser;
/// Module responsible for the report generation
/// both the text and html reports
mod report;

use cli::Cli;
use quake3_parser::parser::{scan_file, Game};
use report::get_report;

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

    let result = get_report(&games, &cli.report_type, &cli.report_format);
    match result {
        Ok(term_table) => match &cli.output_file {
            Some(output_file) => {
                let output_str = term_table.to_string();
                fs::write(output_file, output_str).expect("Error writing file");
            }
            None => println!("{term_table}"),
        },
        Err(err) => eprintln!("Could not generate report: {err}"),
    }
}
