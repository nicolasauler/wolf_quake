//! Quake 3 log parser

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

use std::{collections::HashMap, fmt::Display, fs, path::Path};

#[derive(Debug, Clone)]
enum MeanDeath {
    Unknown,
    Shotgun,
    Gauntlet,
    Machinegun,
    Grenade,
    GrenadeSplash,
    Rocket,
    RocketSplash,
    Plasma,
    PlasmaSplash,
    Railgun,
    Lightning,
    Bfg,
    BfgSplash,
    Water,
    Slime,
    Lava,
    Crush,
    Telefrag,
    Falling,
    Suicide,
    TargetLaser,
    TriggerHurt,
    Nail,
    Chaingun,
    ProximityMine,
    Kamikaze,
    Juiced,
    Grapple,
}

impl From<u32> for MeanDeath {
    fn from(id: u32) -> Self {
        match id {
            1 => Self::Shotgun,
            2 => Self::Gauntlet,
            3 => Self::Machinegun,
            4 => Self::Grenade,
            5 => Self::GrenadeSplash,
            6 => Self::Rocket,
            7 => Self::RocketSplash,
            8 => Self::Plasma,
            9 => Self::PlasmaSplash,
            10 => Self::Railgun,
            11 => Self::Lightning,
            12 => Self::Bfg,
            13 => Self::BfgSplash,
            14 => Self::Water,
            15 => Self::Slime,
            16 => Self::Lava,
            17 => Self::Crush,
            18 => Self::Telefrag,
            19 => Self::Falling,
            20 => Self::Suicide,
            21 => Self::TargetLaser,
            22 => Self::TriggerHurt,
            23 => Self::Nail,
            24 => Self::Chaingun,
            25 => Self::ProximityMine,
            26 => Self::Kamikaze,
            27 => Self::Juiced,
            28 => Self::Grapple,
            _ => Self::Unknown,
        }
    }
}

impl Display for MeanDeath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Unknown => write!(f, "Unknown"),
            Self::Shotgun => write!(f, "Shotgun"),
            Self::Gauntlet => write!(f, "Gauntlet"),
            Self::Machinegun => write!(f, "Machinegun"),
            Self::Grenade => write!(f, "Grenade"),
            Self::GrenadeSplash => write!(f, "Grenade Splash"),
            Self::Rocket => write!(f, "Rocket"),
            Self::RocketSplash => write!(f, "Rocket Splash"),
            Self::Plasma => write!(f, "Plasma"),
            Self::PlasmaSplash => write!(f, "Plasma Splash"),
            Self::Railgun => write!(f, "Railgun"),
            Self::Lightning => write!(f, "Lightning"),
            Self::Bfg => write!(f, "Bfg"),
            Self::BfgSplash => write!(f, "Bfg Splash"),
            Self::Water => write!(f, "Water"),
            Self::Slime => write!(f, "Slime"),
            Self::Lava => write!(f, "Lava"),
            Self::Crush => write!(f, "Crush"),
            Self::Telefrag => write!(f, "Telefrag"),
            Self::Falling => write!(f, "Falling"),
            Self::Suicide => write!(f, "Suicide"),
            Self::TargetLaser => write!(f, "TargetLaser"),
            Self::TriggerHurt => write!(f, "TriggerHurt"),
            Self::Nail => write!(f, "Nail"),
            Self::Chaingun => write!(f, "Chaingun"),
            Self::ProximityMine => write!(f, "ProximityMine"),
            Self::Kamikaze => write!(f, "Kamikaze"),
            Self::Juiced => write!(f, "Juiced"),
            Self::Grapple => write!(f, "Grapple"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlayerData {
    name: String,
    kills: u32,
}

impl PartialOrd for PlayerData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kills.cmp(&other.kills)
    }
}

#[derive(Debug)]
struct Game {
    total_kills: Vec<MeanDeath>,
    players_data: HashMap<u32, PlayerData>,
}

const WORLD_ID: u32 = 1022;

/// parses the `ClientConnect` event and initializes the `players_data`
fn parse_client_connect<'part, I>(
    parts: &mut I,
    players_data: &mut HashMap<u32, PlayerData>,
) -> Result<(), ParsingError>
where
    I: Iterator<Item = &'part str>,
{
    let client_id = parts
        .next()
        .ok_or(ParsingError::NotFound("client_id".to_owned()))?
        .parse::<u32>()?;
    players_data.entry(client_id).or_insert_with(|| PlayerData {
        name: "unknown".to_owned(),
        kills: 0,
    });

    Ok(())
}

/// parses the `ClientUserinfoChanged` event and updates the `players_data`
/// with the player name
fn parse_user_info<'part, I>(
    parts: &mut I,
    players_data: &mut HashMap<u32, PlayerData>,
) -> Result<(), ParsingError>
where
    I: Iterator<Item = &'part str>,
{
    let client_id = parts
        .next()
        .ok_or(ParsingError::NotFound("client_id".to_owned()))?
        .parse::<u32>()?;
    let name = parts.collect::<Vec<&str>>().join(" ");
    let name = name
        .chars()
        .skip(2)
        .take_while(|&c| c != '\\')
        .collect::<String>();
    players_data
        .get_mut(&client_id)
        .expect("Player not found")
        .name = name;

    Ok(())
}

enum ParsingError {
    /// When an expected value from the log is not found
    /// (e.g. the `mean_id` in the Kill event)
    NotFound(String),
    /// When the parsing of a u32 fails
    /// (e.g. when parsing the `killer_id` in the Kill event)
    /// (`std::num::ParseIntError`)
    ParseIntError(std::num::ParseIntError),
    /// When an IO error occurs
    /// (e.g. when reading the file, if the filepath is invalid)
    IoError(std::io::Error),
}

impl From<std::num::ParseIntError> for ParsingError {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<std::io::Error> for ParsingError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(s) => write!(f, "Not found: {s}"),
            Self::ParseIntError(err) => write!(f, "ParseIntError: {err}"),
            Self::IoError(err) => write!(f, "IoError: {err}"),
        }
    }
}

/// parses the Kill event and updates the `players_data`
/// with the number of kills
/// as well as the `total_kills` vector with the mean of death
///
/// can error if the parsing of the u32 fails (`std::num::ParseIntError`)
///
fn parse_kill<'part, I>(
    parts: &mut I,
    players_data: &mut HashMap<u32, PlayerData>,
    total_kills: &mut Vec<MeanDeath>,
) -> Result<(), ParsingError>
where
    I: Iterator<Item = &'part str>,
{
    let killer_id = parts
        .next()
        .ok_or(ParsingError::NotFound("killer_id".to_owned()))?
        .parse::<u32>()?;
    let victim_id = parts
        .next()
        .ok_or(ParsingError::NotFound("victim_id".to_owned()))?
        .parse::<u32>()?;

    let mean_id_text = parts
        .next()
        .ok_or(ParsingError::NotFound("mean_id".to_owned()))?;
    // removing the last character (that is a colon) from the mean_id_text
    let mean_id = mean_id_text[..mean_id_text.len().saturating_sub(1)].parse::<u32>()?;
    total_kills.push(MeanDeath::from(mean_id));

    if killer_id == WORLD_ID {
        let data = players_data.get_mut(&victim_id).expect("Player not found");
        data.kills = data.kills.saturating_sub(1);
    } else {
        let data = players_data.get_mut(&killer_id).expect("Player not found");
        data.kills = data.kills.saturating_add(1);
    }

    Ok(())
}

/// scans the file and returns a vector of games
/// each game contains a vector of `total_kills` and a hashmap of `players_data`
/// the `players_data` hashmap contains the player id as key and the player data as value
fn scan_file(filepath: &Path) -> Result<Vec<Game>, ParsingError> {
    let log_str = fs::read_to_string(filepath)?;

    let mut games: Vec<Game> = Vec::new();
    let mut total_kills: Vec<MeanDeath> = Vec::new();
    let mut players_data: HashMap<u32, PlayerData> = HashMap::new();

    for line in log_str.lines() {
        let mut parts = line.split_whitespace();
        let (_time, event) = (
            parts
                .next()
                .ok_or(ParsingError::NotFound("timestamp".to_owned()))?,
            parts
                .next()
                .ok_or(ParsingError::NotFound("event".to_owned()))?,
        );

        match event {
            "ShutdownGame:" => {
                games.push(Game {
                    total_kills: total_kills.clone(),
                    players_data: players_data.clone(),
                });
                players_data.clear();
                total_kills.clear();
            }
            "ClientConnect:" => {
                parse_client_connect(&mut parts, &mut players_data)?;
            }
            "ClientUserinfoChanged:" => {
                parse_user_info(&mut parts, &mut players_data)?;
            }
            "Kill:" => {
                parse_kill(&mut parts, &mut players_data, &mut total_kills)?;
            }
            _ => {}
        }
    }

    Ok(games)
}

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
