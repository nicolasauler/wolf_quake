use super::errors::ParsingError;
use crate::quake3_data::{MeanDeath, PlayerData, WORLD_ID};
use std::collections::HashMap;

/// Represents a game with the total kills and the players data
#[derive(Debug)]
pub struct Game {
    /// The total kills in the game, counts also world kills
    /// is represented by a vector of `MeanDeath`
    pub total_kills: Vec<MeanDeath>,
    /// The players data in the game
    /// is represented by a hashmap with the player id as key and the player data as value
    /// the player data contains the player name and the number of kills
    /// the number of kills is decremented when the player is killed by the world
    pub players_data: HashMap<u32, PlayerData>,
}

fn finish_game_and_set_new_game(
    games: &mut Vec<Game>,
    total_kills: &mut Vec<MeanDeath>,
    players_data: &mut HashMap<u32, PlayerData>,
) {
    games.push(Game {
        total_kills: total_kills.clone(),
        players_data: players_data.clone(),
    });
    players_data.clear();
    total_kills.clear();
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
        .ok_or_else(|| ParsingError::LogPartNotFound("client_id".to_owned()))?
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
        .ok_or_else(|| ParsingError::LogPartNotFound("client_id".to_owned()))?
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
        .ok_or_else(|| ParsingError::LogPartNotFound("killer_id".to_owned()))?
        .parse::<u32>()?;
    let victim_id = parts
        .next()
        .ok_or_else(|| ParsingError::LogPartNotFound("victim_id".to_owned()))?
        .parse::<u32>()?;

    let mean_id_text = parts
        .next()
        .ok_or_else(|| ParsingError::LogPartNotFound("mean_id".to_owned()))?;
    // removing the last character (that is a colon) from the mean_id_text
    if mean_id_text.len() <= 1 {
        return Err(ParsingError::LogPartNotFound("mean_id".to_owned()));
    }
    let mean_id = mean_id_text[..mean_id_text.len().saturating_sub(1)].parse::<u32>()?;
    total_kills.push(MeanDeath::from(mean_id));

    if killer_id == WORLD_ID {
        let data = players_data
            .get_mut(&victim_id)
            .ok_or_else(|| ParsingError::UnexpectedError("Victim not found".to_owned()))?;
        data.kills = data.kills.saturating_sub(1);
    } else {
        let data = players_data
            .get_mut(&killer_id)
            .ok_or_else(|| ParsingError::UnexpectedError("Killer not found".to_owned()))?;
        data.kills = data.kills.saturating_add(1);
    }

    Ok(())
}

/// scans the file and returns a vector of games
/// each game contains a vector of `total_kills` and a hashmap of `players_data`
/// the `players_data` hashmap contains the player id as key and the player data as value
pub fn scan_file(log_content: &str) -> Result<Vec<Game>, ParsingError> {
    let mut games: Vec<Game> = Vec::new();
    let mut total_kills: Vec<MeanDeath> = Vec::new();
    let mut players_data: HashMap<u32, PlayerData> = HashMap::new();

    for line in log_content.lines() {
        let mut parts = line.split_whitespace();
        let time = if let Some(timestamp) = parts.next() {
            timestamp
        } else {
            // skip empty lines
            continue;
        };
        if time.len() < 4 || !(time.chars().all(|c| c.is_numeric() || c == ':')) {
            // skip lines that don't start with a timestamp
            continue;
        }
        let event = parts
            .next()
            .ok_or_else(|| ParsingError::LogPartNotFound("event".to_owned()))?;

        match event {
            "InitGame:" => {
                if !total_kills.is_empty() {
                    finish_game_and_set_new_game(&mut games, &mut total_kills, &mut players_data);
                }
            }
            "ShutdownGame:" => {
                finish_game_and_set_new_game(&mut games, &mut total_kills, &mut players_data);
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn arb_player_data()(name in "[a-z]*", kills in any::<u32>()) -> PlayerData {
            PlayerData { name, kills }
        }
    }

    proptest! {
        #[test]
        fn test_parse_client_connect(
            client_connect_line in (any::<u32>().prop_map(|v| v.to_string())),
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10)
        ) {
            let mut parts = client_connect_line.split_whitespace();
            let client_id = parts.clone().next().unwrap().parse::<u32>().unwrap();

            if players_data.contains_key(&client_id) {
                let result = parse_client_connect(&mut parts, &mut players_data);
                prop_assert!(result.is_ok());
                prop_assert!(players_data.contains_key(&client_id));
                prop_assert_ne!(players_data.get(&client_id).unwrap(), &PlayerData { name: "unknown".to_owned(), kills: 0 });
            }
            else {
                let result = parse_client_connect(&mut parts, &mut players_data);
                prop_assert!(result.is_ok());
                prop_assert!(players_data.contains_key(&client_id));
                prop_assert_eq!(players_data.get(&client_id).unwrap(), &PlayerData { name: "unknown".to_owned(), kills: 0 });
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_client_connect_part_not_found(
            client_connect_line in "\\s*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10)
        ) {
            let mut parts = client_connect_line.split_whitespace();

            let result = parse_client_connect(&mut parts, &mut players_data);
            match result {
                Err(ParsingError::LogPartNotFound(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_client_connect_parseint_error(
            client_connect_line in "[^\\d\\s]+", // match everything that is not a digit or a whitespace
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10)
        ) {
            let mut parts = client_connect_line.split_whitespace();

            let result = parse_client_connect(&mut parts, &mut players_data);
            match result {
                Err(ParsingError::ParseIntError(_)) => {},
                _ => {
                    prop_assert!(false)
                },
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_user_info(
            client_id in any::<u32>(),
            two_chars in "[\\S]{2}",
            name in "\\w*",
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10)
        ) {
            let user_info_line = format!("{} {}{}\\{}", client_id, two_chars, name, rest);
            players_data.insert(client_id, PlayerData { name: "unknown".to_owned(), kills: 0 });

            let mut parts = user_info_line.split_whitespace();
            let client_id = parts.clone().next().unwrap().parse::<u32>().unwrap();

            let result = parse_user_info(&mut parts, &mut players_data);
            prop_assert!(result.is_ok());
            prop_assert!(players_data.contains_key(&client_id));
            prop_assert_ne!(players_data.get(&client_id).unwrap(), &PlayerData { name: "unknown".to_owned(), kills: 0 });
            prop_assert_eq!(players_data.get(&client_id).unwrap(), &PlayerData { name: name.to_owned(), kills: 0 });
        }
    }

    proptest! {
        #[test]
        fn test_parse_user_info_part_not_found(
            user_info_line in "\\s*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10)
        ) {
            let mut parts = user_info_line.split_whitespace();

            let result = parse_user_info(&mut parts, &mut players_data);
            match result {
                Err(ParsingError::LogPartNotFound(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_user_info_parseint_error(
            client_id in "[^\\d\\s]+", // match everything that is not a digit or a whitespace
            two_chars in "\\PC*",
            name in "\\PC*",
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10)
        ) {
            let user_info_line = format!("{} {}{}\\{}", client_id, two_chars, name, rest);
            let mut parts = user_info_line.split_whitespace();

            let result = parse_user_info(&mut parts, &mut players_data);
            match result {
                Err(ParsingError::ParseIntError(_)) => {},
                _ => {
                    prop_assert!(false)
                },
            }
        }
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

    proptest! {
        #[test]
        fn test_parse_kill(
            killer_id in any::<u32>(),
            victim_id in any::<u32>(),
            mean_id in 0..28u32,
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
            mut total_kills in prop::collection::vec(a_random_mean_death(), 1),
        ) {
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            players_data.insert(killer_id, PlayerData { name: "unknown".to_owned(), kills: 0 });
            players_data.insert(victim_id, PlayerData { name: "unknown".to_owned(), kills: 1 });

            let mut parts = kill_line.split_whitespace();
            let killer_id = parts.clone().next().unwrap().parse::<u32>().unwrap();
            let victim_id = parts.clone().nth(1).unwrap().parse::<u32>().unwrap();
            let mean_text = parts.clone().nth(2).unwrap();
            // remove the last character (that is a colon) from the mean_text
            let mean_id = mean_text[..mean_text.len().saturating_sub(1)].parse::<u32>().unwrap();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            prop_assert!(result.is_ok());

            if killer_id == WORLD_ID {
                prop_assert_eq!(players_data.get(&victim_id).unwrap().kills, 0);
            }
            else {
                prop_assert_eq!(players_data.get(&killer_id).unwrap().kills, 1);
            }

            prop_assert_eq!(total_kills.len(), 2);
            prop_assert_eq!(total_kills.last().unwrap(), &MeanDeath::from(mean_id));
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_mean_id_not_found(
            killer_id in any::<u32>(),
            victim_id in any::<u32>(),
            mean_id in "\\s*",
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::LogPartNotFound(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_victim_id_not_found(
            killer_id in any::<u32>(),
            victim_id in "\\s*",
            mean_id in 0..28u32,
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::ParseIntError(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_killer_id_not_found(
            killer_id in "\\s*",
            victim_id in any::<u32>(),
            mean_id in 0..28u32,
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::ParseIntError(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_mean_id_parseint_error(
            killer_id in any::<u32>(),
            victim_id in any::<u32>(),
            mean_id in "[^\\d\\s]+", // match everything that is not a digit or a whitespace
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::ParseIntError(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_victim_id_parseint_error(
            killer_id in any::<u32>(),
            victim_id in "[^\\d\\s]+", // match everything that is not a digit or a whitespace
            mean_id in 0..28u32,
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::ParseIntError(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_killer_id_parseint_error(
            killer_id in "[^\\d\\s]+", // match everything that is not a digit or a whitespace
            victim_id in any::<u32>(),
            mean_id in 0..28u32,
            rest in "\\PC*",
            mut players_data in prop::collection::hash_map(any::<u32>(), arb_player_data(), 0..10),
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::ParseIntError(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_killer_not_found_unexpected_error(
            killer_id in any::<u32>(),
            victim_id in any::<u32>(),
            mean_id in 0..28u32,
            rest in "\\PC*",
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            prop_assume!(killer_id != victim_id);

            let mut players_data: HashMap<u32, PlayerData> = HashMap::new();
            players_data.insert(victim_id, PlayerData { name: "unknown".to_owned(), kills: 1 });
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::UnexpectedError(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    proptest! {
        #[test]
        fn test_parse_kill_victim_not_found_unexpected_error(
            victim_id in any::<u32>(),
            mean_id in 0..28u32,
            rest in "\\PC*",
            mut total_kills in prop::collection::vec(a_random_mean_death(), 0..10),
        ) {
            let killer_id = WORLD_ID;
            prop_assume!(killer_id != victim_id);

            let mut players_data: HashMap<u32, PlayerData> = HashMap::new();
            players_data.insert(killer_id, PlayerData { name: "unknown".to_owned(), kills: 1 });
            let kill_line = format!("{} {} {}: {}", killer_id, victim_id, mean_id, rest);
            let mut parts = kill_line.split_whitespace();

            let result = parse_kill(&mut parts, &mut players_data, &mut total_kills);
            match result {
                Err(ParsingError::UnexpectedError(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }

    #[test]
    fn test_scan_file() {
        let log_content = r#"
        0:00 ------------------------------------------------------------
        0:00 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\bot_minplayers\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
        0:01 ClientConnect: 2
        0:02 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
        0:03 ClientConnect: 3
        0:04 ClientUserinfoChanged: 3 n\Mocinha\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\95\w\0\l\0\tt\0\tl\0
        0:05 Kill: 2 3 7: Isgalamido killed Mocinha by MOD_ROCKET_SPLASH
        0:06 Kill: 3 2 7: Mocinha killed Isgalamido by MOD_ROCKET_SPLASH
        0:07 ShutdownGame:
        0:07 ------------------------------------------------------------
        0:08 ------------------------------------------------------------
        0:08 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\bot_minplayers\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
        0:09 ClientConnect: 2
        0:10 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
        0:11 Kill: 2 2 22: Isgalamido killed Isgalamido by MOD_TRIGGER_HURT
        0:12 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT
        0:13 ShutdownGame:
        0:14 ------------------------------------------------------------
        "#;

        let games = scan_file(log_content).unwrap();
        assert_eq!(games.len(), 2);

        let game0 = &games[0];
        assert_eq!(game0.total_kills.len(), 2);
        assert_eq!(game0.players_data.len(), 2);
        assert_eq!(game0.players_data.get(&2).unwrap().name, "Isgalamido");
        assert_eq!(game0.players_data.get(&2).unwrap().kills, 1);
        assert_eq!(game0.players_data.get(&3).unwrap().name, "Mocinha");
        assert_eq!(game0.players_data.get(&3).unwrap().kills, 1);

        let game1 = &games[1];
        assert_eq!(game1.total_kills.len(), 2);
        assert_eq!(game1.players_data.len(), 1);
        assert_eq!(game1.players_data.get(&2).unwrap().name, "Isgalamido");
        assert_eq!(game1.players_data.get(&2).unwrap().kills, 0);
    }

    proptest! {
        #[test]
        fn test_scan_file_prop(
            whatever in "\\PC*",
            player1_id in any::<u32>(),
            player2_id in any::<u32>(),
            mean_id in 0..28u32,
        ) {
            let log_content = format!(
                r#"
                0:00 ------------------------------------------------------------
                0:00 InitGame: {whatever}
                0:01 ClientConnect: {player1_id}
                0:02 ClientUserinfoChanged: {player1_id} n\Isgalamido\{whatever}
                0:03 ClientConnect: {player2_id}
                0:04 ClientUserinfoChanged: {player2_id} n\Mocinha\{whatever}
                0:05 Kill: {player1_id} {player2_id} {mean_id}: {whatever}
                0:06 Kill: {player2_id} {player1_id} {mean_id}: {whatever}
                0:07 ShutdownGame:
                0:07 ------------------------------------------------------------
                "#,
            );

            let games = scan_file(&log_content).unwrap();
            assert_eq!(games.len(), 1);

            let game0 = &games[0];
            assert_eq!(game0.total_kills.len(), 2);
            assert_eq!(game0.players_data.len(), 2);
            assert_eq!(game0.players_data.get(&player1_id).unwrap().name, "Isgalamido");
            assert_eq!(game0.players_data.get(&player1_id).unwrap().kills, 1);
            assert_eq!(game0.players_data.get(&player2_id).unwrap().name, "Mocinha");
            assert_eq!(game0.players_data.get(&player2_id).unwrap().kills, 1);
        }
    }

    #[test]
    fn test_buggy_scan_file() {
        let log_content = r#"
        0:00 ------------------------------------------------------------
        0:00 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\bot_minplayers\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
        0:01 ClientConnect: 2
        0:02 ClientUserinfoChanged: 2 n\Dono da bola\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
        0:03 ClientConnect: 3
        0:04 ClientUserinfoChanged: 3 n\Mocinha\t\0\model\sarge\hmodel\sarge\g_redteam\\g_blueteam\\c1\4\c2\5\hc\95\w\0\l\0\tt\0\tl\0
        0:05 Kill: 2 3 7: Dono da bola killed Mocinha by MOD_ROCKET_SPLASH
        0:06 Kill: 3 2 7: Mocinha killed Dono da bola by MOD_ROCKET_SPLASH
        0:07 ShutdownGame:
        26 0:07 ------------------------------------------------------------
        0:08 InitGame: \sv_floodProtect\1\sv_maxPing\0\sv_minPing\0\sv_maxRate\10000\sv_minRate\0\sv_hostname\Code Miner Server\g_gametype\0\sv_privateClients\2\sv_maxclients\16\sv_allowDownload\0\bot_minplayers\0\dmflags\0\fraglimit\20\timelimit\15\g_maxGameClients\0\capturelimit\8\version\ioq3 1.36 linux-x86_64 Apr 12 2009\protocol\68\mapname\q3dm17\gamename\baseq3\g_needpass\0
        0:09 ClientConnect: 2
        0:10 ClientUserinfoChanged: 2 n\Isgalamido\t\0\model\uriel/zael\hmodel\uriel/zael\g_redteam\\g_blueteam\\c1\5\c2\5\hc\100\w\0\l\0\tt\0\tl\0
        0:11 Kill: 2 2 22: Isgalamido killed Isgalamido by MOD_TRIGGER_HURT
        0:12 ShutdownGame:
        0:13 ------------------------------------------------------------
        "#;

        let games = scan_file(log_content).unwrap();
        assert_eq!(games.len(), 2);

        let game0 = &games[0];
        assert_eq!(game0.total_kills.len(), 2);
        assert_eq!(game0.players_data.len(), 2);
        assert_eq!(game0.players_data.get(&2).unwrap().name, "Dono da bola");
        assert_eq!(game0.players_data.get(&2).unwrap().kills, 1);
        assert_eq!(game0.players_data.get(&3).unwrap().name, "Mocinha");
        assert_eq!(game0.players_data.get(&3).unwrap().kills, 1);

        let game1 = &games[1];
        assert_eq!(game1.total_kills.len(), 1);
        assert_eq!(game1.players_data.len(), 1);
        assert_eq!(game1.players_data.get(&2).unwrap().name, "Isgalamido");
        assert_eq!(game1.players_data.get(&2).unwrap().kills, 1);
    }

    proptest! {
        #[test]
        fn test_scan_file_event_not_found(
            event in "\\s*",
            whatever in "\\PC*",
        ) {
            let log_content = format!(
                r#"
                0:00 ------------------------------------------------------------
                0:00 InitGame: {whatever}
                0:01 ClientConnect: 2
                0:02 {event}
                "#,
            );

            let result = scan_file(&log_content);
            match result {
                Err(ParsingError::LogPartNotFound(_)) => {},
                _ => prop_assert!(false),
            }
        }
    }
}
