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

pub fn parse_steam_id(data: &[u8]) -> Result<[u8; 8], Box<dyn Error>> {
    Ok(data[STEAM_ID_LOCATION..][..8].try_into()?)
}

pub fn get_slot_start_position(character_slot_index: usize) -> usize {
    SLOT_START_INDEX + character_slot_index * 0x10 + character_slot_index * SLOT_LENGTH
}

pub fn get_header_start_position(character_slot_index: usize) -> usize {
    SAVE_HEADER_START_INDEX + character_slot_index * SAVE_HEADER_LENGTH
}

/// Elden Ring stores name as an array of bytes,
/// each followed by a null byte
#[derive(Debug, Clone)]
pub struct NameString {
    data: Vec<u8>,
}

impl std::fmt::Display for NameString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8_lossy(
                self.data
                    .iter()
                    .filter(|b| **b != 0)
                    .copied()
                    .collect::<Vec<u8>>()
                    .as_slice()
            )
        )
    }
}

#[test]
fn name_string() {
    let hello_world_w_nulls = vec![
        72, 0, 101, 0, 108, 0, 108, 0, 111, 0, 32, 0, 87, 0, 111, 0, 114, 0, 108, 0, 100, 0,
    ];
    let ns = NameString {
        data: hello_world_w_nulls,
    };
    assert_eq!(ns.to_string(), "Hello World");
}

#[derive(Debug, Clone)]
pub struct Character {
    pub index: usize,
    pub active: bool,
    pub name: NameString,
    pub save_data: Vec<u8>,
    pub header_data: Vec<u8>,
    pub slot_start_index: usize,
    pub header_start_index: usize,
}

fn parse_active(data: &[u8], index: usize) -> bool {
    data.iter().skip(CHAR_ACTIVE_STATUS_START_INDEX).nth(index) == Some(&1)
}

fn parse_name(data: &[u8], index: usize) -> NameString {
    let name_bytes = data
        .iter()
        .skip(SAVE_HEADER_START_INDEX + index * SAVE_HEADER_LENGTH)
        .take(CHAR_NAME_LENGTH)
        .copied()
        .collect();
    NameString { data: name_bytes }
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
    fn new(data: &[u8], index: usize) -> Self {
        Self {
            index,
            active: parse_active(data, index),
            name: parse_name(data, index),
            save_data: parse_save_data(data, index),
            header_data: parse_header_data(data, index),
            slot_start_index: get_slot_start_position(index),
            header_start_index: get_header_start_position(index),
        }
    }

    /// Generate Character iff the Character is active
    pub fn new_active(data: &[u8], index: usize) -> Option<Self> {
        let active = parse_active(data, index);
        active.then(|| Self::new(data, index))
    }
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Slot {}: {}", self.index, self.name)
    }
}
