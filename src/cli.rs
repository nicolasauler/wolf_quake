use clap::{Parser, ValueEnum};
use tabled::{
    builder::Builder,
    settings::{object::Segment, Alignment, Settings, Style},
    Table,
};

use crate::{
    quake3_data::{MeanDeath, PlayerData},
    quake3_parser::parser::Game,
};
use std::path::PathBuf;

#[derive(Clone, Debug, ValueEnum)]
/// Type of report to generate:
/// - Report with player ranking and mean of death ranking
/// - Report with player ranking
/// - Report with mean of death ranking
pub enum ReportType {
    /// Player kill score ranking + mean of death ranking
    All,
    /// Only player kill score ranking
    PlayerRank,
    /// Only mean of death ranking
    MeanDeath,
}

#[derive(Clone, Debug, ValueEnum)]
/// Format of report to generate:
/// - Text table report in console
/// - Html table report
pub enum ReportFormat {
    /// HTML table report
    Html,
    /// Text console report with tabled crate
    Text,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// The CLI struct
/// Defines the declarative CLI interface using the `clap` crate
pub struct Cli {
    /// The path to the log file, required
    pub log_file: PathBuf,

    #[arg(short, long, value_enum, default_value = "all")]
    /// The type of report to generate
    /// - Report with player ranking and mean of death ranking
    /// - Report with player ranking
    /// - Report with mean of death ranking
    /// Default: all
    pub report_type: ReportType,

    #[arg(short = 'f', long, value_enum, default_value = "text")]
    /// The format of the report to generate
    /// - Text table report in console
    /// - Html table report
    /// Default: text
    pub report_format: ReportFormat,
}

/// now, for each game, we will print:
/// Game N | Total kills in game: X | Player with most kills: Y
///                                          ...
///                                   Player with less kills: Z
/// Game N+1 | Total kills in game: X | Player with most kills: Y
///                                         ...
///                                     Player with less kills: Z
pub fn term_report(games: &[Game], report_type: &ReportType) -> Result<Table, &'static str> {
    let mut builder = Builder::default();
    let mut game_number = games.len();

    for game in games.iter().rev() {
        let players_data: &mut Vec<&PlayerData> = &mut game.players_data.values().collect();
        players_data.sort_unstable();

        let mut m_data = String::new();
        let mut kills_by_means_death: Vec<(&MeanDeath, &u32)> =
            game.kills_by_means_death.iter().collect();
        kills_by_means_death.sort_unstable_by(|a, b| b.1.cmp(a.1));
        for (mean, count) in &kills_by_means_death {
            m_data.push_str(&format!("\n{mean}: {count}\n"));
        }

        let mut p_data = String::new();
        for player in players_data.iter() {
            p_data.push_str(&format!("\n{}: {}\n", player.name, player.kills));
        }

        let mut game_data = vec![
            format!("Game {}", game_number),
            format!("{}", game.total_kills),
        ];
        match report_type {
            ReportType::All => {
                game_data.push(p_data);
                game_data.push(m_data);
            }
            ReportType::PlayerRank => {
                game_data.push(p_data);
            }
            ReportType::MeanDeath => {
                game_data.push(m_data);
            }
        }
        builder.insert_record(0, game_data);
        game_number = game_number.checked_sub(1).ok_or("Game number is zero")?;
    }

    let mut columns = vec!["\n\n", "\nTotal game kills\n"];
    match report_type {
        ReportType::All => {
            columns.push("\nKill Rank\n(Player: Score)\n");
            columns.push("\nDeath Causes\n(Cause: Count)\n");
        }
        ReportType::PlayerRank => {
            columns.push("\nKill Rank\n(Player: Score)\n");
        }
        ReportType::MeanDeath => {
            columns.push("\nDeath Causes\n(Cause: Count)\n");
        }
    }

    builder.insert_record(0, columns);
    let mut table = builder.build();
    table.with(Style::modern_rounded());
    table.modify(
        Segment::all(),
        Settings::new(Alignment::center(), Alignment::center_vertical()),
    );

    Ok(table)
}
