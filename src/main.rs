//! Quake 3 log parser

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod quake3_data;
mod quake3_parser;

use quake3_parser::parser::{scan_file, Game};
use std::path::Path;

#[cfg_attr(coverage_nightly, coverage(off))]
/// main function
fn main() {
    let filepath = Path::new("./static/qgames.log");
    let games: Vec<Game> = match scan_file(filepath) {
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
