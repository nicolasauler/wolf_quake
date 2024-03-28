use super::errors::ParsingError;
use crate::quake3_data::{MeanDeath, PlayerData, WORLD_ID};
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug)]
pub struct Game {
    pub total_kills: Vec<MeanDeath>,
    pub players_data: HashMap<u32, PlayerData>,
}

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
pub fn scan_file(filepath: &Path) -> Result<Vec<Game>, ParsingError> {
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
