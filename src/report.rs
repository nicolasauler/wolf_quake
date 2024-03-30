use table_to_html::HtmlTable;
use tabled::{
    builder::Builder,
    settings::{object::Segment, Alignment, Settings, Style},
    Table,
};

use crate::{
    cli::{ReportFormat, ReportType},
    quake3_data::{MeanDeath, PlayerData},
    quake3_parser::parser::Game,
};
use std::fmt::Display;

#[allow(clippy::large_enum_variant)]
// I think size difference isn't actually that big
// even though Table seem to be bigger
// and Boxing Table seems excessive
// let's keep it for now
#[derive(Debug)]
/// The report type
/// Can be a text table or an html table
pub enum Report {
    /// Text table report, via the `tabled` crate
    Text(Table),
    /// Html table report, via the `table_to_html` crate
    Html(HtmlTable),
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(table) => write!(f, "{table}"),
            Self::Html(html_table) => write!(f, "{html_table}"),
        }
    }
}

/// Populates the table content rows for the terminal report
/// with the game data, player data and means of death data
fn populate_table_content(
    builder: &mut Builder,
    game: &Game,
    players_data: &[&PlayerData],
    report_type: &ReportType,
    game_number: usize,
) {
    let mut m_data = String::new();
    let mut kills_by_means_death: Vec<(&MeanDeath, &u32)> =
        game.kills_by_means_death.iter().collect();
    kills_by_means_death.sort_unstable_by(|a, b| b.1.cmp(a.1));
    for (mean, count) in &kills_by_means_death {
        m_data.push_str(&format!("\n{mean}: {count}\n"));
    }

    let mut p_data = String::new();
    for player in players_data {
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
}

/// Populates the table headers for the terminal report
/// with the columns for the report type
fn populate_table_headers(builder: &mut Builder, report_type: &ReportType) {
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
}

/// Returns report with the game data, player data and means of death data
/// in a table format
///
/// The report can be either text or html
///
/// And can include or exclude the player ranking and the mean of death ranking
///
/// The report format is as follows:
/// Game N | Total kills in game: X | Player with most kills: Y
///                                          ...
///                                   Player with less kills: Z
/// Game N+1 | Total kills in game: X | Player with most kills: Y
///                                         ...
///                                     Player with less kills: Z
pub fn get_report(
    games: &[Game],
    report_type: &ReportType,
    report_format: &ReportFormat,
) -> Result<Report, &'static str> {
    let mut builder = Builder::default();
    let mut game_number = games.len();

    for game in games.iter().rev() {
        let players_data: &mut Vec<&PlayerData> = &mut game.players_data.values().collect();
        players_data.sort_unstable();

        populate_table_content(&mut builder, game, players_data, report_type, game_number);

        game_number = game_number.checked_sub(1).ok_or("Game number is zero")?;
    }
    populate_table_headers(&mut builder, report_type);

    match report_format {
        ReportFormat::Text => {
            let mut table = builder.build();
            table.with(Style::modern_rounded());
            table.modify(
                Segment::all(),
                Settings::new(Alignment::center(), Alignment::center_vertical()),
            );
            Ok(Report::Text(table))
        }
        ReportFormat::Html => {
            let mut html_table = HtmlTable::with_header(Vec::<Vec<String>>::from(builder));
            html_table.set_alignment(
                table_to_html::Entity::Global,
                table_to_html::Alignment::center(),
            );
            html_table.set_border(1);
            Ok(Report::Html(html_table))
        }
    }
}
