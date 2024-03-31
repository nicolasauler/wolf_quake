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
#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_display_empty_text_report() {
        let report = Report::Text(Table::default());
        let report_str = report.to_string();
        assert!(report_str.is_empty());
    }

    #[test]
    fn test_display_empty_html_report() {
        let report = Report::Html(HtmlTable::new(vec![vec![""]]));
        let report_str = report.to_string();
        let expected = r#"<table>
    <tbody>
        <tr>
            <td>
                <div>
                    <p>
                        
                    </p>
                </div>
            </td>
        </tr>
    </tbody>
</table>"#;
        assert_eq!(report_str, expected);
    }

    /// Since the display of Report only passed to the formatter of each variant
    /// let's test the `tabled` directly with: https://github.com/zhiburt/tabled/tree/master?tab=readme-ov-file#usage
    #[test]
    fn test_display_report_text_table() {
        struct Language {
            name: &'static str,
            designed_by: &'static str,
            invented_year: usize,
        }

        let languages = vec![
            Language {
                name: "C",
                designed_by: "Dennis Ritchie",
                invented_year: 1972,
            },
            Language {
                name: "Go",
                designed_by: "Rob Pike",
                invented_year: 2009,
            },
            Language {
                name: "Rust",
                designed_by: "Graydon Hoare",
                invented_year: 2010,
            },
        ];

        let mut builder = Builder::new();
        for language in languages.iter().rev() {
            let record = vec![
                language.name.to_string(),
                language.designed_by.to_string(),
                language.invented_year.to_string(),
            ];
            builder.insert_record(0, record);
        }
        let columns = vec!["name", "designed_by", "invented_year"];
        builder.insert_record(0, columns);
        let table = builder.build();
        let report = Report::Text(table);

        let expected = "+------+----------------+---------------+\n\
                | name | designed_by    | invented_year |\n\
                +------+----------------+---------------+\n\
                | C    | Dennis Ritchie | 1972          |\n\
                +------+----------------+---------------+\n\
                | Go   | Rob Pike       | 2009          |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  | 2010          |\n\
                +------+----------------+---------------+";

        assert_eq!(report.to_string(), expected);
    }

    /// Since the display of Report only passed to the formatter of each variant
    /// let's test the `table_to_html` directly with: https://docs.rs/table_to_html/latest/table_to_html/#example-building-a-table-from-iterator
    #[test]
    fn test_display_report_html_table() {
        let data = vec![
            vec!["Debian", "", "0"],
            vec!["Arch", "", "0"],
            vec!["Manjaro", "Arch", "0"],
        ];

        let html_table = HtmlTable::new(data);
        let report = Report::Html(html_table);

        assert_eq!(
            report.to_string(),
            concat!(
                "<table>\n",
                "    <tbody>\n",
                "        <tr>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        Debian\n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        \n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        0\n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "        </tr>\n",
                "        <tr>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        Arch\n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        \n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        0\n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "        </tr>\n",
                "        <tr>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        Manjaro\n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        Arch\n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "            <td>\n",
                "                <div>\n",
                "                    <p>\n",
                "                        0\n",
                "                    </p>\n",
                "                </div>\n",
                "            </td>\n",
                "        </tr>\n",
                "    </tbody>\n",
                "</table>"
            ),
        )
    }

    fn a_random_mean_death() -> impl Strategy<Value = MeanDeath> {
        prop_oneof![
            Just(MeanDeath::Unknown),
            Just(MeanDeath::Shotgun),
            Just(MeanDeath::Gauntlet),
            Just(MeanDeath::Machinegun),
            Just(MeanDeath::Grenade),
            Just(MeanDeath::GrenadeSplash),
            Just(MeanDeath::Rocket),
            Just(MeanDeath::RocketSplash),
            Just(MeanDeath::Plasma),
            Just(MeanDeath::PlasmaSplash),
            Just(MeanDeath::Railgun),
            Just(MeanDeath::Lightning),
            Just(MeanDeath::Bfg),
            Just(MeanDeath::BfgSplash),
            Just(MeanDeath::Water),
            Just(MeanDeath::Slime),
            Just(MeanDeath::Lava),
            Just(MeanDeath::Crush),
            Just(MeanDeath::Telefrag),
            Just(MeanDeath::Falling),
            Just(MeanDeath::Suicide),
            Just(MeanDeath::TargetLaser),
            Just(MeanDeath::TriggerHurt),
            Just(MeanDeath::Nail),
            Just(MeanDeath::Chaingun),
            Just(MeanDeath::ProximityMine),
            Just(MeanDeath::Kamikaze),
            Just(MeanDeath::Juiced),
            Just(MeanDeath::Grapple),
        ]
    }

    prop_compose! {
        fn arb_player_data()(name in "[a-z]*", kills in any::<i32>()) -> PlayerData {
            PlayerData { name, kills }
        }
    }

    prop_compose! {
        fn arb_game()(
            total_kills in any::<u32>(),
            kills_by_means_death in prop::collection::hash_map(a_random_mean_death(), any::<u32>(), 0..10),
            players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
        ) -> Game {
            Game {
                total_kills,
                kills_by_means_death,
                players_data
            }
        }
    }

    fn report_type() -> impl Strategy<Value = ReportType> {
        prop_oneof![
            Just(ReportType::All),
            Just(ReportType::PlayerRank),
            Just(ReportType::MeanDeath),
        ]
    }

    fn report_format() -> impl Strategy<Value = ReportFormat> {
        prop_oneof![Just(ReportFormat::Html), Just(ReportFormat::Text)]
    }

    proptest! {
        #[test]
        fn test_get_report(
            games in prop::collection::vec(arb_game(), 1..5),
            report_type in report_type(),
            report_format in report_format(),
        ) {
            let result = get_report(&games, &report_type, &report_format);
            assert!(result.is_ok());

            match result {
                Ok(Report::Text(table)) => {
                    let table_str = table.to_string();
                    assert!(!table_str.is_empty());
                }
                Ok(Report::Html(html_table)) => {
                    let html_table_str = html_table.to_string();
                    assert!(!html_table_str.is_empty());
                }
                _ => panic!("Unexpected result"),
            }
        }
    }

    #[test]
    fn test_get_simple_report() {
        let mut kills_by_means_death: HashMap<MeanDeath, u32> = HashMap::new();
        kills_by_means_death.insert(MeanDeath::TriggerHurt, 1);
        let mut players_data: HashMap<u32, PlayerData> = HashMap::new();
        players_data.insert(
            2,
            PlayerData {
                name: "Player1".to_string(),
                kills: -1,
            },
        );

        let games = vec![
            Game {
                total_kills: 1,
                kills_by_means_death: kills_by_means_death.clone(),
                players_data: players_data.clone(),
            },
            Game {
                total_kills: 1,
                kills_by_means_death,
                players_data,
            },
        ];

        let report_type = ReportType::All;
        let report_format = ReportFormat::Text;
        let result = get_report(&games, &report_type, &report_format);
        assert!(result.is_ok());

        let expected = concat!(
            "╭────────┬──────────────────┬─────────────────┬────────────────╮\n",
            "│        │                  │                 │                │\n",
            "│        │ Total game kills │ Kill Rank       │ Death Causes   │\n",
            "│        │                  │ (Player: Score) │ (Cause: Count) │\n",
            "│        │                  │                 │                │\n",
            "├────────┼──────────────────┼─────────────────┼────────────────┤\n",
            "│        │                  │                 │                │\n",
            "│ Game 1 │        1         │   Player1: -1   │ TriggerHurt: 1 │\n",
            "│        │                  │                 │                │\n",
            "├────────┼──────────────────┼─────────────────┼────────────────┤\n",
            "│        │                  │                 │                │\n",
            "│ Game 2 │        1         │   Player1: -1   │ TriggerHurt: 1 │\n",
            "│        │                  │                 │                │\n",
            "╰────────┴──────────────────┴─────────────────┴────────────────╯",
        );

        let table_str = result.unwrap().to_string();
        assert!(!table_str.is_empty());
        assert_eq!(table_str, expected);
    }

    proptest! {
        #[test]
        fn test_populate_table_content(
            game in arb_game(),
            report_type in report_type(),
            game_number in any::<usize>(),
        ) {
            let mut builder = Builder::default();
            let players_data: &mut Vec<&PlayerData> = &mut game.players_data.values().collect();
            players_data.sort_unstable();
            populate_table_content(&mut builder, &game, players_data, &report_type, game_number);
            let table = builder.build();
            let table_str = table.to_string();
            assert!(!table_str.is_empty());
        }
    }

    #[test]
    fn test_simple_populate_table_content() {
        let mut kills_by_means_death: HashMap<MeanDeath, u32> = HashMap::new();
        kills_by_means_death.insert(MeanDeath::TriggerHurt, 1);
        let mut players_data: HashMap<u32, PlayerData> = HashMap::new();
        players_data.insert(
            2,
            PlayerData {
                name: "Player1".to_string(),
                kills: -1,
            },
        );

        let game = Game {
            total_kills: 1,
            kills_by_means_death,
            players_data,
        };

        let report_type = ReportType::All;
        let game_number = 1;
        let mut builder = Builder::default();
        let players_data: &mut Vec<&PlayerData> = &mut game.players_data.values().collect();
        players_data.sort_unstable();
        populate_table_content(&mut builder, &game, players_data, &report_type, game_number);
        let mut table = builder.build();
        table.with(Style::modern_rounded());
        let table_str = table.to_string();
        assert!(!table_str.is_empty());

        // formatting seems weird because we didn't set the alignment
        let expected = concat!(
            "╭────────┬───┬─────────────┬────────────────╮\n",
            "│ Game 1 │ 1 │             │                │\n",
            "│        │   │ Player1: -1 │ TriggerHurt: 1 │\n",
            "│        │   │             │                │\n",
            "╰────────┴───┴─────────────┴────────────────╯",
        );

        assert_eq!(table_str, expected);
    }

    proptest! {
        #[test]
        fn test_populate_table_headers(
            report_type in report_type(),
        ) {
            let mut builder = Builder::default();
            populate_table_headers(&mut builder, &report_type);
            let table = builder.build();
            let table_str = table.to_string();
            assert!(!table_str.is_empty());
        }
    }

    #[test]
    fn test_simple_populate_table_headers() {
        let report_type = ReportType::All;
        let mut builder = Builder::default();
        populate_table_headers(&mut builder, &report_type);
        let mut table = builder.build();
        table.with(Style::modern_rounded());
        let table_str = table.to_string();
        assert!(!table_str.is_empty());

        let expected = concat!(
            "╭──┬──────────────────┬─────────────────┬────────────────╮\n",
            "│  │                  │                 │                │\n",
            "│  │ Total game kills │ Kill Rank       │ Death Causes   │\n",
            "│  │                  │ (Player: Score) │ (Cause: Count) │\n",
            "│  │                  │                 │                │\n",
            "╰──┴──────────────────┴─────────────────┴────────────────╯",
        );

        assert_eq!(table_str, expected);
    }
}
