use std::fmt::Display;

/// The world id is hardcoded as 1022
pub const WORLD_ID: u32 = 1022;

#[derive(Debug, Clone, PartialEq, Eq)]
/// The player data struct
/// Contains the player name and the number of kills
/// The number of kills is decremented when the world kills a player
///
/// Implements custom `PartialOrd` and `Ord` to allow sorting by the number of kills
pub struct PlayerData {
    /// The player name
    pub name: String,
    /// The player score
    pub kills: i32,
}

impl PartialOrd for PlayerData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerData {
    /// Sorts by the number of kills in descending order
    /// The player with the most kills is first
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.kills.cmp(&self.kills)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(clippy::missing_docs_in_private_items)]
/// The means of death enum
/// Contains the possible means of death in Quake 3
pub enum MeanDeath {
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    prop_compose! {
        fn arb_player_data_pos()(name in "[a-z]*", kills in 0..i32::MAX) -> PlayerData {
            PlayerData { name, kills }
        }
    }

    prop_compose! {
        fn arb_players_pos()
        (player_data in arb_player_data_pos())
        (name in "[a-z]*", kills in 0..player_data.kills, player_data in Just(player_data))
        -> (PlayerData, PlayerData) {
            (player_data, PlayerData { name, kills })
        }
    }

    prop_compose! {
        fn arb_player_data_neg()(name in "[a-z]*", kills in i32::MIN..0) -> PlayerData {
            PlayerData { name, kills }
        }
    }

    prop_compose! {
        fn arb_players_neg()
        (player_data in arb_player_data_neg())
        (name in "[a-z]*", kills in player_data.kills..0, player_data in Just(player_data))
        -> (PlayerData, PlayerData) {
            (player_data, PlayerData { name, kills })
        }
    }

    proptest! {
        #[test]
        fn test_player_data_ordering_pos((a_player, other_player) in arb_players_pos()) {
            prop_assert!(a_player < other_player);
        }
    }

    proptest! {
        #[test]
        fn test_player_data_ordering_neg((a_player, other_player) in arb_players_neg()) {
            prop_assert!(a_player > other_player);
        }
    }

    proptest! {
        #[test]
        fn test_player_data_ordering_follows_kills_pos((a_player, other_player) in arb_players_pos()) {
            prop_assert_eq!(a_player.cmp(&other_player), other_player.kills.cmp(&a_player.kills));
        }
    }

    proptest! {
        #[test]
        fn test_player_data_ordering_follows_kills_neg((a_player, other_player) in arb_players_neg()) {
            prop_assert_eq!(a_player.cmp(&other_player), other_player.kills.cmp(&a_player.kills));
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
        fn test_mean_death_from_u32(mean in a_random_mean_death()) {
            let mean_number = mean.clone() as u32;
            prop_assert_eq!(MeanDeath::from(mean_number), mean);
        }
    }

    #[test]
    fn test_display_mean_death() {
        assert_eq!(MeanDeath::Unknown.to_string(), "Unknown");
        assert_eq!(MeanDeath::Shotgun.to_string(), "Shotgun");
        assert_eq!(MeanDeath::Gauntlet.to_string(), "Gauntlet");
        assert_eq!(MeanDeath::Machinegun.to_string(), "Machinegun");
        assert_eq!(MeanDeath::Grenade.to_string(), "Grenade");
        assert_eq!(MeanDeath::GrenadeSplash.to_string(), "Grenade Splash");
        assert_eq!(MeanDeath::Rocket.to_string(), "Rocket");
        assert_eq!(MeanDeath::RocketSplash.to_string(), "Rocket Splash");
        assert_eq!(MeanDeath::Plasma.to_string(), "Plasma");
        assert_eq!(MeanDeath::PlasmaSplash.to_string(), "Plasma Splash");
        assert_eq!(MeanDeath::Railgun.to_string(), "Railgun");
        assert_eq!(MeanDeath::Lightning.to_string(), "Lightning");
        assert_eq!(MeanDeath::Bfg.to_string(), "Bfg");
        assert_eq!(MeanDeath::BfgSplash.to_string(), "Bfg Splash");
        assert_eq!(MeanDeath::Water.to_string(), "Water");
        assert_eq!(MeanDeath::Slime.to_string(), "Slime");
        assert_eq!(MeanDeath::Lava.to_string(), "Lava");
        assert_eq!(MeanDeath::Crush.to_string(), "Crush");
        assert_eq!(MeanDeath::Telefrag.to_string(), "Telefrag");
        assert_eq!(MeanDeath::Falling.to_string(), "Falling");
        assert_eq!(MeanDeath::Suicide.to_string(), "Suicide");
        assert_eq!(MeanDeath::TargetLaser.to_string(), "TargetLaser");
        assert_eq!(MeanDeath::TriggerHurt.to_string(), "TriggerHurt");
        assert_eq!(MeanDeath::Nail.to_string(), "Nail");
        assert_eq!(MeanDeath::Chaingun.to_string(), "Chaingun");
        assert_eq!(MeanDeath::ProximityMine.to_string(), "ProximityMine");
        assert_eq!(MeanDeath::Kamikaze.to_string(), "Kamikaze");
        assert_eq!(MeanDeath::Juiced.to_string(), "Juiced");
        assert_eq!(MeanDeath::Grapple.to_string(), "Grapple");
    }
}
