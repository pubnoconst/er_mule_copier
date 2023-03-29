use std::process::exit;

use clap::Parser;
use comfy_table::Table;
use er_mule_copier::{file_io, save_model};
/// Simple program to copy Elden Ring save file
#[derive(Parser, Debug)]
struct Args {
    /// Input save file
    #[arg(short, long)]
    source: String,
    /// Target save file
    #[arg(short, long)]
    target: String,
}

fn main() {
    let args = Args::parse();

    let source_data = std::fs::read(args.source).unwrap();
    let source_characters = file_io::list_characters(&source_data);

    let target_data = std::fs::read(&args.target).unwrap();
    let target_characters = file_io::list_characters(&target_data);

    let mut table = Table::new();
    table.set_header(vec!["Source", "Target"]);
    for i in 0..10 {
        let source_column = match &source_characters[i] {
            Some(c) => c.to_string(),
            None => format!("Slot {i}: (Empty)"),
        };
        let target_column = match &target_characters[i] {
            Some(c) => c.to_string(),
            None => format!("Slot {i}: (Empty)"),
        };
        table.add_row(vec![source_column, target_column]);
    }
    println!("{table}");

    let source_index: usize;
    let target_index: usize;
    println!("Enter source slot followed by the target slot separated by a space (i.e. \"2 5\"):");
    text_io::scan!("{} {}", source_index, target_index);
    if !(0..10).contains(&source_index) {
        eprintln!("Error: Source index out of range.");
        exit(1);
    }
    if !(0..10).contains(&target_index) {
        eprintln!("Error: Target index out of range.");
        exit(1);
    }

    let source_character = save_model::Character::new_active(&source_data, source_index);
    if source_character.is_none() {
        eprintln!("Cannot copy from an empty slot");
        exit(1);
    }
    let source_character = source_character.unwrap();

    match file_io::generate_new_data(&source_data, &source_character, &target_data, target_index) {
        Ok(new_save) => match file_io::write_backup(&new_save, None) {
            Ok(pb) => {
                println!("Backup written successfully as {:?}", pb);
                match file_io::write_file(&new_save, &args.target.into()) {
                    Ok(_) => println!("File overwritten successfully -- Done!"),
                    Err(e) => {
                        eprintln!("Error overwriting save file: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed making backup ({e}), exiting.");
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("Failed generating new save ({e}), exiting.");
            exit(1);
        }
    }
}
