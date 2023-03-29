use std::{
    error::Error,
    fs::{self, remove_file},
    path::PathBuf,
};

use crate::save_model::{self, Character};

pub fn list_active_characters(data: &[u8]) -> Vec<Character> {
    (0..11)
        .filter_map(|index| Character::new_active(data, index))
        .collect()
}

pub fn list_characters(data: &[u8]) -> Vec<Option<Character>> {
    (0..11)
        .map(|index| Character::new_active(data, index))
        .collect()
}

// timestamp as backup identifier
fn get_unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// writes backup to destination path
// if destination is None, writes into the Documents folder
// returns backup file name
pub fn write_backup(data: &[u8], destination: Option<&PathBuf>) -> Result<PathBuf, Box<dyn Error>> {
    let mut pb = match destination {
        Some(pb) => pb.to_owned(),
        None => match dirs_next::document_dir() {
            Some(doc) => {
                let mut pb: PathBuf = doc;
                pb.push("er_mule_copier_backups"); // backup folder
                fs::create_dir_all(&pb)?;
                pb
            }
            None => return Err("Unable to find backup destination file".into()),
        },
    };
    pb.push(format!("ER0000 backup from {}", get_unix_timestamp()).as_str());
    pb.set_extension("sl2");
    std::fs::write(&pb, data)?;
    Ok(pb)
}

pub fn write_file(data: &[u8], fully_qualified_file_name: &PathBuf) -> Result<(), Box<dyn Error>> {
    // backup file needs to be deleted for possible file corruption error
    let mut backup_file_name = fully_qualified_file_name.clone();
    backup_file_name.set_extension("bak");
    if let Err(e) = remove_file(&backup_file_name) {
        eprintln!(
            "{:?} was not removed: {}. Make sure there is no .bak file in the game folder ",
            backup_file_name, e
        );
    }

    // write
    std::fs::write(fully_qualified_file_name, data)?;
    Ok(())
}

pub fn generate_new_data(
    source_data: &[u8],
    source_character: &Character,
    target_data: &[u8],
    target_slot_index: usize,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut result = target_data.to_owned();
    let source_steam_id = save_model::parse_steam_id(source_data)?;
    let target_steam_id = save_model::parse_steam_id(target_data)?;

    // Replace steam id in the source Character
    // with the steam id of the target Character
    // in the target Character save_data
    let mut result_character = source_character.clone();
    let mut i = 0;
    while i + 8 < result_character.save_data.len() {
        if result_character.save_data[i..i + 8] == source_steam_id {
            result_character.save_data[i..i + 8].copy_from_slice(&target_steam_id);
        }
        i += 1;
    }

    let target_character_slot_start_index = save_model::get_slot_start_position(target_slot_index);
    let target_character_header_start_index =
        save_model::get_header_start_position(target_slot_index);

    // write new character save_data
    result[target_character_slot_start_index..][..save_model::SLOT_LENGTH]
        .copy_from_slice(&result_character.save_data);

    // write new character header_data
    result[target_character_header_start_index..][..save_model::SAVE_HEADER_LENGTH]
        .copy_from_slice(&result_character.header_data);

    // mark new character active
    result[save_model::CHAR_ACTIVE_STATUS_START_INDEX + target_slot_index] = 0x01;

    let mut md5 = md5::Context::new();

    md5.consume(&result_character.save_data);
    let slot_checksum_digest = md5.clone().compute();
    let slot_checksum = slot_checksum_digest.as_slice();
    result[target_character_slot_start_index - 0x10..][..0x10].copy_from_slice(slot_checksum);

    md5.consume(
        &result[save_model::SAVE_HEADERS_SECTION_START_INDEX..]
            [..save_model::SAVE_HEADERS_SECTION_LENGTH],
    );
    let header_checksum_digest = md5.compute();
    let header_checksum = header_checksum_digest.as_slice();

    result[save_model::SAVE_HEADERS_SECTION_START_INDEX - 0x10..][..0x10]
        .copy_from_slice(header_checksum);

    Ok(result)
}
