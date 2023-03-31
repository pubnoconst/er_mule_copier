use std::process::exit;

use clap::Parser;
use comfy_table::Table;
use er_mule_copier::file_io;

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

    let source_data = std::fs::read(args.source);
    if source_data.is_err() {
        eprintln!("Unable to read source file.");
        exit(1);
    }
    let source_data = source_data.unwrap();

    let target_data = std::fs::read(&args.target);
    if target_data.is_err() {
        eprintln!("Unable to read target file.");
        exit(1);
    }
    let target_data = target_data.unwrap();

    let target_characters = file_io::list_characters(&target_data);
    let source_characters = file_io::list_characters(&source_data);

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

    let source_slot: usize;
    let target_slot: usize;
    println!("Enter source slot followed by the target slot separated by a space (i.e. \"2 5\"):");
    text_io::scan!("{} {}", source_slot, target_slot);
    if !(0..10).contains(&source_slot) {
        eprintln!("Error: Source index out of range.");
        exit(1);
    }
    if !(0..10).contains(&target_slot) {
        eprintln!("Error: Target index out of range.");
        exit(1);
    }

    match file_io::generate_new_data(&source_data, source_slot, &target_data, target_slot) {
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
