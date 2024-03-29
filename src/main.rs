//! Quake 3 log parser

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

/// Module responsible for the data representation from the log
/// like the means of death and the players data
/// the `PlayerData` struct and the `MeanDeath` enum
mod quake3_data;
/// Module responsible for the parsing functionalities
mod quake3_parser;

use quake3_parser::parser::{scan_file, Game};
use std::{fs, path::Path};

#[cfg_attr(coverage_nightly, coverage(off))]
/// main function
fn main() {
    let filepath = Path::new("./static/qgames.log");
    let log_str = fs::read_to_string(filepath).expect("Error reading file");

    let games: Vec<Game> = match scan_file(&log_str) {
        Ok(games) => games,
        Err(err) => {
            eprintln!("Error parsing file {filepath:?}: {err}");
            return;
        }
    };

    for game in games {
        let total_kills = game.total_kills;
        println!("Total kills: {total_kills:?}");
        let players_data = game.players_data;
        println!("Players data: {players_data:?}");
    }
}
