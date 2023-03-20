pub const SLOT_START_INDEX: usize = 0x310;
pub const SLOT_LENGTH: usize = 0x280000;
pub const SAVE_HEADERS_SECTION_START_INDEX: usize = 0x19003B0;
pub const SAVE_HEADERS_SECTION_LENGTH: usize = 0x60000;
pub const SAVE_HEADER_START_INDEX: usize = 0x1901D0E;
pub const SAVE_HEADER_LENGTH: usize = 0x24C;
pub const CHAR_ACTIVE_STATUS_START_INDEX: usize = 0x1901D04;
pub const CHAR_NAME_LENGTH: usize = 0x22;
pub const CHAR_LEVEL_LOCATION: usize = 0x22;
pub const CHAR_PLAYED_START_INDEX: usize = 0x26;
pub const STEAM_ID_LOCATION: usize = 0x19003B4;

pub struct SteamID {
    pub data: [u8; 8],
}

impl SteamID {
    pub fn parse_from_game_data(data: &[u8]) -> Self {
        Self {
            data: {
                assert!(data.len() >= STEAM_ID_LOCATION + 8);
                let source_byte_iter = data.iter().skip(STEAM_ID_LOCATION).take(8).copied();
                let mut id_bytes = [0; 8];
                for (i, b) in source_byte_iter.enumerate() {
                    id_bytes[i] = b;
                }
                id_bytes
            },
        }
    }
}

fn parse_active(slot_index: usize, game_data: &[u8]) -> bool {
    game_data
        .iter()
        .skip(CHAR_ACTIVE_STATUS_START_INDEX)
        .nth(slot_index)
        == Some(&1)
}

fn parse_character_name(slot_index: usize, game_data: &[u8]) -> String {
    let bytes = game_data
        .iter()
        .skip(SAVE_HEADER_START_INDEX + (slot_index * SAVE_HEADER_LENGTH))
        .take(CHAR_NAME_LENGTH)
        .filter(|b| **b != 0) // dotnet Encoding.Unicode.GetString seems to implicitly remove null bytes
        .copied()
        .collect::<Vec<u8>>();
    String::from_utf8_lossy(&bytes).to_string()
}

fn parse_save_data(slot_index: usize, game_data: &[u8]) -> Vec<u8> {
    game_data
        .iter()
        .skip(SLOT_START_INDEX + slot_index * 0x10 + slot_index * SLOT_LENGTH)
        .take(SLOT_LENGTH)
        .copied()
        .collect::<Vec<u8>>()
}

fn parse_header_data(slot_index: usize, game_data: &[u8]) -> Vec<u8> {
    game_data
        .iter()
        .skip(SAVE_HEADER_START_INDEX + slot_index * SAVE_HEADER_LENGTH)
        .take(SAVE_HEADER_LENGTH)
        .copied()
        .collect::<Vec<u8>>()
}

fn get_slot_start_index(slot_index: usize) -> usize {
    SLOT_START_INDEX + slot_index * 0x10 + slot_index * SLOT_LENGTH
}

fn get_header_start_index(slot_index: usize) -> usize {
    SAVE_HEADER_START_INDEX + slot_index * SAVE_HEADER_LENGTH
}

// Save is a save slot that contains values below
pub struct Save {
    pub active: bool,
    pub character_name: String,
    pub save_data: Vec<u8>,
    pub header_data: Vec<u8>,
    pub id: uuid::Uuid, // for internal use, has no effect in savefiles
    pub slot_index: usize,
    pub slot_start_index: usize,
    pub header_start_index: usize,
}

impl Save {
    pub fn from_raw_data(slot_index: usize, game_data: &[u8]) -> Self {
        Self {
            active: parse_active(slot_index, game_data),
            character_name: parse_character_name(slot_index, game_data),
            save_data: parse_header_data(slot_index, game_data),
            header_data: parse_header_data(slot_index, game_data),
            id: uuid::Uuid::new_v4(),
            slot_index,
            slot_start_index: get_slot_start_index(slot_index),
            header_start_index: get_header_start_index(slot_index),
        }
    }
}

impl std::fmt::Display for Save {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Slot {}{}{}",
            self.slot_index,
            if self.active { ": " } else { "" },
            if self.active {
                &self.character_name
            } else {
                ""
            }
        )
    }
}
