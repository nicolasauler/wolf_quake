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

#[cfg(test)]
mod tests {
    use self::cli::{ReportFormat, ReportType};

    use super::*;

    #[test]
    fn pseudo_integration_test_imitating_main() {
        let log_str = concat!(
" 0:00 ------------------------------------------------------------\n",
" 0:00 InitGame: \\sv_floodProtect\\1\\sv_maxPing\\0\\sv_minPing\\0\\sv_maxRate\\10000\\sv_minRate\\0\\sv_hostname\\Code Miner Server\\g_gametype\\0\\sv_privateClients\\2\\sv_maxclients\\16\\sv_allowDownload\\0\\dmflags\\0\\fraglimit\\20\\timelimit\\15\\g_maxGameClients\\0\\capturelimit\\8\\version\\ioq3 1.36 linux-x86_64 Apr 12 2009\\protocol\\68\\mapname\\q3dm17\\gamename\\baseq3\\g_needpass\\0\n",
" 20:34 ClientConnect: 2\n",
" 20:34 ClientUserinfoChanged: 2 n\\Player1\\t\\0\\model\\\\xian/default\\hmodel\\xian/default\\g_redteam\\\\g_blueteam\\\\c1\\4\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\0\n",
" 20:37 ClientBegin: 2\n",
" 20:37 Kill: 1022 2 22: <world> killed Player1 by MOD_TRIGGER_HURT\n",
" 20:37 ShutdownGame:\n",
" 20:37 ------------------------------------------------------------\n",
" 20:37 ------------------------------------------------------------\n",
" 0:00 InitGame: \\sv_floodProtect\\1\\sv_maxPing\\0\\sv_minPing\\0\\sv_maxRate\\10000\\sv_minRate\\0\\sv_hostname\\Code Miner Server\\g_gametype\\0\\sv_privateClients\\2\\sv_maxclients\\16\\sv_allowDownload\\0\\dmflags\\0\\fraglimit\\20\\timelimit\\15\\g_maxGameClients\\0\\capturelimit\\8\\version\\ioq3 1.36 linux-x86_64 Apr 12 2009\\protocol\\68\\mapname\\q3dm17\\gamename\\baseq3\\g_needpass\\0\n",
" 20:34 ClientConnect: 2\n",
" 20:34 ClientUserinfoChanged: 2 n\\Player1\\t\\0\\model\\\\xian/default\\hmodel\\xian/default\\g_redteam\\\\g_blueteam\\\\c1\\4\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\0\n",
" 20:37 ClientBegin: 2\n",
" 20:34 ClientConnect: 3\n",
" 20:34 ClientUserinfoChanged: 3 n\\Player2\\t\\0\\model\\\\xian/default\\hmodel\\xian/default\\g_redteam\\\\g_blueteam\\\\c1\\4\\c2\\5\\hc\\100\\w\\0\\l\\0\\tt\\0\\tl\\0\n",
" 20:37 ClientBegin: 3\n",
" 20:37 Kill: 1022 2 22: <world> killed Player1 by MOD_TRIGGER_HURT\n",
" 20:37 Kill: 3 2 7: Player2 killed Player1 by MOD_ROCKET_SPLASH\n",
" 20:37 Kill: 3 2 7: Player2 killed Player1 by MOD_ROCKET_SPLASH\n",
" 20:37 ShutdownGame:\n",
" 20:37 ------------------------------------------------------------",
        );

        let games = scan_file(&log_str).unwrap();
        let result = get_report(&games, &ReportType::All, &ReportFormat::Text).unwrap();

        let expected = concat!(
            "╭────────┬──────────────────┬─────────────────┬──────────────────╮\n",
            "│        │                  │                 │                  │\n",
            "│        │ Total game kills │ Kill Rank       │  Death Causes    │\n",
            "│        │                  │ (Player: Score) │  (Cause: Count)  │\n",
            "│        │                  │                 │                  │\n",
            "├────────┼──────────────────┼─────────────────┼──────────────────┤\n",
            "│        │                  │                 │                  │\n",
            "│ Game 1 │        1         │   Player1: -1   │  TriggerHurt: 1  │\n",
            "│        │                  │                 │                  │\n",
            "├────────┼──────────────────┼─────────────────┼──────────────────┤\n",
            "│        │                  │                 │                  │\n",
            "│        │                  │   Player2: 2    │ Rocket Splash: 2 │\n",
            "│ Game 2 │        3         │                 │                  │\n",
            "│        │                  │   Player1: -1   │ TriggerHurt: 1   │\n",
            "│        │                  │                 │                  │\n",
            "╰────────┴──────────────────┴─────────────────┴──────────────────╯",
        );

        assert_eq!(result.to_string(), expected);
    }
}
