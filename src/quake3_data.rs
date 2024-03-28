use std::fmt::Display;

pub const WORLD_ID: u32 = 1022;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerData {
    pub name: String,
    pub kills: u32,
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

#[derive(Debug, Clone)]
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
