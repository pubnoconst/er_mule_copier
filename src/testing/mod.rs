#[cfg(test)]

const RAWFILE_PATH_STR: &str = "src/testing/ER0000.sl2";

#[test]
fn list_saves() {
    let saves = crate::file_io::list_saves(&std::fs::read(RAWFILE_PATH_STR).unwrap());
    for save in saves {
        println!("{} :: {}", save.slot_index, save.character_name);
    }
    assert!(true);
}

#[test]
fn locate_slices() {
    assert_eq!(
        crate::file_io::locate_slices(&vec![1, 1, 1, 1, 1, 1, 1], &vec![1, 1, 1]),
        vec![0, 3]
    );
    assert!(true);
}

#[test]
fn parse_steam_id() {
    let game_data = std::fs::read(RAWFILE_PATH_STR).unwrap();
    let steam_id = crate::save_model::SteamID::parse_from_game_data(&game_data);
    println!("Steam ID: {:?}", steam_id.data);
    assert!(true);
}

#[test]
fn replace_bytes() {
    let mut src = [0, 1, 2, 3, 4, 5, 6, 7];
    let with = [55; 6];
    crate::helpers::replace_bytes(&mut src, 1, &with, 2);
    assert_eq!(src, [0, 55, 55, 3, 4, 5, 6, 7]);
}