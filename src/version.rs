#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Version {
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    Unknown,
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::V1,
            2 => Self::V2,
            3 => Self::V3,
            4 => Self::V4,
            5 => Self::V5,
            6 => Self::V6,
            7 => Self::V7,
            _ => Self::Unknown,
        }
    }
}
