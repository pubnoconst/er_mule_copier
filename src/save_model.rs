use std::error::Error;

pub const SLOT_START_INDEX: usize = 0x310;
pub const SLOT_LENGTH: usize = 0x280000;
pub const SAVE_HEADERS_SECTION_START_INDEX: usize = 0x19003B0;
pub const SAVE_HEADERS_SECTION_LENGTH: usize = 0x60000;
pub const SAVE_HEADER_START_INDEX: usize = 0x1901D0E;
pub const SAVE_HEADER_LENGTH: usize = 0x24C;
pub const CHAR_ACTIVE_STATUS_START_INDEX: usize = 0x1901D04;
pub const CHAR_NAME_LENGTH: usize = 0x22;
pub const STEAM_ID_LOCATION: usize = 0x19003B4;
pub const STEAM_ID_LENGTH: usize = 8;

pub fn parse_steam_id(data: &[u8]) -> Result<[u8; STEAM_ID_LENGTH], Box<dyn Error>> {
    Ok(data[STEAM_ID_LOCATION..][..STEAM_ID_LENGTH].try_into()?)
}

pub fn get_slot_start_position(character_slot_index: usize) -> usize {
    SLOT_START_INDEX + character_slot_index * 0x10 + character_slot_index * SLOT_LENGTH
}

pub fn get_header_start_position(character_slot_index: usize) -> usize {
    SAVE_HEADER_START_INDEX + character_slot_index * SAVE_HEADER_LENGTH
}

pub struct Character {
    pub index: usize,
    pub active: bool,
    pub name: String,
}

fn parse_active(data: &[u8], index: usize) -> bool {
    data.iter().skip(CHAR_ACTIVE_STATUS_START_INDEX).nth(index) == Some(&1)
}

pub fn parse_name(data: &[u8], index: usize) -> String {
    let name_bytes = data
        .iter()
        .skip(SAVE_HEADER_START_INDEX + index * SAVE_HEADER_LENGTH)
        .take(CHAR_NAME_LENGTH)
        .copied()
        .filter(|b| *b != 0)
        .collect::<Vec<u8>>();
    String::from_utf8_lossy(&name_bytes).to_string()
}

pub fn parse_save_data(data: &[u8], index: usize) -> Vec<u8> {
    data.iter()
        .skip(SLOT_START_INDEX + index * 0x10 + index * SLOT_LENGTH)
        .take(SLOT_LENGTH)
        .copied()
        .collect()
}

pub fn parse_header_data(data: &[u8], index: usize) -> Vec<u8> {
    data.iter()
        .skip(SAVE_HEADER_START_INDEX + index * SAVE_HEADER_LENGTH)
        .take(SAVE_HEADER_LENGTH)
        .copied()
        .collect()
}

impl Character {
    /// Generate Character from data
    /// WILL contain inactive characters
    /// Call in context of target characters
    pub fn new(data: &[u8], index: usize) -> Self {
        Self {
            index,
            active: parse_active(data, index),
            name: parse_name(data, index),
        }
    }

    /// Generate Character iff the Character is active
    /// Call in context of source
    pub fn new_active(data: &[u8], index: usize) -> Option<Self> {
        let active = parse_active(data, index);
        active.then(|| Self::new(data, index))
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot {} {}", self.index, self.name)
    }
}
