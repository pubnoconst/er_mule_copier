use std::{
    error::Error,
    fs::remove_file,
    path::PathBuf,
};

use crate::save_model::{self, Character};

pub fn list_active_characters(data: &[u8]) -> Vec<Character> {
    (0..10)
        .filter_map(|index| Character::new_active(data, index))
        .collect()
}

pub fn list_characters(data: &[u8]) -> Vec<Option<Character>> {
    (0..10)
        .map(|index| Character::new_active(data, index))
        .collect()
}

/// Prases all chraracters, active or inactive
pub fn list_all_characters(data: &[u8]) -> Vec<Character> {
    (0..10).map(|i| Character::new(data, i)).collect()
}


pub fn write_file(data: &[u8], fully_qualified_file_name: &PathBuf) -> Result<(), Box<dyn Error>> {
    std::fs::write(fully_qualified_file_name, data)?;
    Ok(())
}

fn subslice_positions(needle: &[u8], haystack: &[u8]) -> Vec<usize> {
    haystack
        .windows(needle.len())
        .enumerate()
        .filter(|(_, slice)| slice[..] == needle[..])
        .map(|(i, _)| i)
        .collect()
}

// We are taking owenership because the upstream does it
// Might address this  later if there is a reason for it
pub fn generate_new_data(
    source_data: &[u8],
    source_character_slot: usize,
    target_data: &[u8],
    target_character_slot: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut new_save = target_data.to_owned();

    let source_id = save_model::parse_steam_id(source_data)?;
    let target_id = save_model::parse_steam_id(target_data)?;

    let mut source_character_save_data =
        save_model::parse_save_data(source_data, source_character_slot);

    let source_character_header_data =
        save_model::parse_header_data(source_data, source_character_slot);

    for id_location in subslice_positions(&source_id, &source_character_save_data) {
        source_character_save_data[id_location..id_location + save_model::STEAM_ID_LENGTH]
            .copy_from_slice(&target_id);
    }

    // Copy source save slot to target save slot in temp file
    new_save[save_model::get_slot_start_position(target_character_slot)..]
        [..save_model::SLOT_LENGTH]
        .copy_from_slice(&source_character_save_data);

    // Copy save header to temp file
    new_save[save_model::get_header_start_position(target_character_slot)..]
        [..save_model::SAVE_HEADER_LENGTH]
        .copy_from_slice(&source_character_header_data);

    // Actiate target slot
    new_save[save_model::CHAR_ACTIVE_STATUS_START_INDEX + target_character_slot] = 1;

    let mut md5 = md5::Context::new();

    md5.consume(source_character_save_data);
    let slot_checksum_digest = md5.compute();
    let slot_checksum = slot_checksum_digest.as_slice();
    new_save[save_model::get_slot_start_position(target_character_slot) - 0x10..][..0x10]
        .copy_from_slice(slot_checksum);

    // reset hasher
    let mut md5 = md5::Context::new();
    md5.consume(
        &new_save[save_model::SAVE_HEADERS_SECTION_START_INDEX..]
            [..save_model::SAVE_HEADERS_SECTION_LENGTH],
    );
    let header_checksum_digest = md5.compute();
    let header_checksum = header_checksum_digest.as_slice();
    new_save[save_model::SAVE_HEADERS_SECTION_START_INDEX - 0x10..][..0x10]
        .copy_from_slice(header_checksum);

    Ok(new_save)
}
