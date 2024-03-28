//! Quake 3 log parser

#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![allow(dead_code)]

use serde::{Serialize, Serializer};
use std::{collections::HashMap, fmt::Display, fs, path::Path};

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug)]
enum LogEvent {
    InitGame,
    ClientConnect {
        client_id: u32,
    },
    ClientUserinfoChanged {
        client_id: u32,
        name: String,
    },
    Kill {
        killer_id: u32,
        killed_id: u32,
        mean_id: u32,
        killer_name: String,
        killed_name: String,
        mean_name: MeanDeath,
    },
    ShutdownGame,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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

#[derive(Debug, Serialize)]
struct Game {
    #[serde(serialize_with = "len_serialize")]
    total_kills: Vec<MeanDeath>,
    players_data: HashMap<u32, PlayerData>,
}

fn len_serialize<S>(v: &[MeanDeath], s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u64(v.len().try_into().unwrap())
}

const WORLD_ID: u32 = 1022;

fn scan_file(filepath: &Path) -> Result<Vec<Game>, std::io::Error> {
    let log_str = fs::read_to_string(filepath).expect("Unable to read file");
    let mut games: Vec<Game> = Vec::new();
    let mut total_kills: Vec<MeanDeath> = Vec::new();
    let mut players_data: HashMap<u32, PlayerData> = HashMap::new();

    for line in log_str.lines() {
        let mut parts = line.split_whitespace();
        let _time = parts.next().unwrap();
        let event = parts.next().unwrap();

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
                let client_id = parts.next().unwrap().parse::<u32>().unwrap();
                players_data.insert(
                    client_id,
                    PlayerData {
                        name: "unknown".to_owned(),
                        kills: 0,
                    },
                );
            }
            "ClientUserinfoChanged:" => {
                let client_id = parts.next().unwrap().parse::<u32>().unwrap();
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
            }
            "Kill:" => {
                let killer_id = parts.next().unwrap().parse::<u32>().unwrap();
                let victim_id = parts.next().unwrap().parse::<u32>().unwrap();

                let mean_id_text = parts.next().unwrap();
                // removing the last character (that is a colon) from the mean_id_text
                let mean_id = mean_id_text[..mean_id_text.len().saturating_sub(1)]
                    .parse::<u32>()
                    .unwrap();
                total_kills.push(MeanDeath::from(mean_id));

                if killer_id == WORLD_ID {
                    let data = players_data.get_mut(&victim_id).expect("Player not found");
                    data.kills = data.kills.saturating_sub(1);
                } else {
                    let data = players_data.get_mut(&killer_id).expect("Player not found");
                    data.kills = data.kills.saturating_add(1);
                }
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
    let games = scan_file(filepath).expect("Unable to scan file");

    let buildin_json = serde_json::json!({
        "games": games.iter().map(|game| {
            let mut players = game.players_data.values().collect::<Vec<&PlayerData>>();
            players.sort_unstable();
            serde_json::json!({
                "total_kills": game.total_kills.len(),
                "players": players.iter().map(|player| {
                    serde_json::json!({
                        "name": player.name,
                        "kills": player.kills,
                    })
                }).collect::<Vec<serde_json::Value>>(),
            })
        }).collect::<Vec<serde_json::Value>>(),
    });

    println!("{}", serde_json::to_string_pretty(&buildin_json).unwrap());
    //println!("{}", serde_json::to_string_pretty(&games).unwrap());
}
