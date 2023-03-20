use crate::{
    helpers,
    save_model::{self, Save, SteamID},
};

pub fn list_saves(game_data: &[u8]) -> Vec<save_model::Save> {
    let mut saves = Vec::with_capacity(10);
    for slot_index in 1..11 {
        let save = save_model::Save::from_raw_data(slot_index, &game_data);
        saves.push(save);
    }
    saves
}

/// Finds the indices of subslices equaling to `needle` within `haystack`.
/// Original implementation does NOT skip `needle.len()` when `needle` is found.
pub fn locate_slices(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    // ugly code ahead
    assert!(haystack.len() >= needle.len());
    let mut result = Vec::new();
    let mut i = 0;
    while i + needle.len() <= haystack.len() {
        if haystack[i..i + needle.len()] == needle[..] {
            result.push(i);
            i += needle.len();
        } else {
            i += 1;
        }
    }
    result
}

pub fn overwrite_steam_id(game_data: &mut [u8], steam_id: &SteamID) {
    for starting_index in locate_slices(&game_data, &steam_id.data) {
        for i in starting_index..starting_index + &steam_id.data.len() {
            game_data[i] = steam_id.data[i - starting_index];
        }
    }
}

// TODO refactor
pub fn overwrite_checksums(
    source_save: &Save,
    target_save: &Save,
    generated_save_content: &mut [u8],
) {
    use md5::Context;
    let mut md5cx = Context::new();
    md5cx.consume(&source_save.save_data);
    let slot_checksum = md5cx.clone().compute();
    helpers::replace_bytes(
        generated_save_content,
        target_save.slot_start_index - 0x10,
        &slot_checksum.0,
        0x10,
    );

    // TODO fix this ugly slicing
    md5cx.consume(
        &generated_save_content[save_model::SAVE_HEADERS_SECTION_START_INDEX
            ..save_model::SAVE_HEADERS_SECTION_START_INDEX
                + save_model::SAVE_HEADERS_SECTION_LENGTH],
    );

    let header_checksum = md5cx.compute();
    helpers::replace_bytes(
        generated_save_content,
        save_model::SAVE_HEADERS_SECTION_START_INDEX - 0x10,
        &header_checksum.0,
        0x10,
    );
}

pub fn generate_new_save_file_content(
    source_save_file_content: &[u8],
    source_save: &Save,
    target_save_file_content: &[u8],
    target_save: &Save,
) -> Vec<u8> {
    // soource_save_content -> Mule file content
    // target_save_content -> Users' existing save file content
    // generated_save_content -> the content of the generated save file as a vector of bytes

    // create new target in memory
    let mut generated_save_content = target_save_file_content.to_vec();
    // overwrite steam id in the memory file
    let target_steam_id = save_model::SteamID::parse_from_game_data(target_save_file_content);
    overwrite_steam_id(&mut generated_save_content, &target_steam_id);
    // copy source save slot into target save slot
    helpers::replace_bytes(
        &mut generated_save_content,
        source_save.slot_start_index,
        source_save_file_content,
        save_model::SLOT_LENGTH,
    );

    // copy header
    helpers::replace_bytes(
        &mut generated_save_content,
        target_save.header_start_index,
        &source_save.header_data,
        save_model::SAVE_HEADER_LENGTH,
    );

    // mark target slot as active
    generated_save_content[save_model::CHAR_ACTIVE_STATUS_START_INDEX + target_save.slot_index] =
        0x01;

    // calculate checksums
    overwrite_checksums(source_save, target_save, &mut generated_save_content);

    // return new generated file
    generated_save_content

    // profit
}
