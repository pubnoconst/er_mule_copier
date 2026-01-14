use std::io::{self, Write};
use std::path::Path;
use comfy_table::Table;
use er_mule_copier::file_io;

struct Args {
    source: String,
    target: String,
}

fn prompt_str(prompt: &str) -> Option<String> {
    print!("{prompt}: ");
    io::stdout().flush().ok()?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok()?;
    let s = buf.trim().trim_matches('"').to_string();

    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}

fn prompt_int(prompt: &str) -> Option<usize> {
    let s = prompt_str(prompt)?;
    s.parse::<usize>().ok()
}

impl Args {
    fn prompt() -> Option<Self> {
        let source = loop {
            let s = prompt_str("Drop SOURCE save file and press Enter (or 'q' to quit)")?;
            if s == "q" {
                return None;
            }
            if Path::new(&s).exists() {
                break s;
            }
            eprintln!("Source file not found.");
        };

        let target = loop {
            let t = prompt_str("Drop TARGET save file and press Enter (or 'q' to quit)")?;
            if t == "q" {
                return None;
            }
            if Path::new(&t).exists() {
                break t;
            }
            eprintln!("Target file not found.");
        };

        Some(Self { source, target })
    }
}

fn greet() {
    println!("Elden Ring Mule Copier");
    println!("This will OVERWRITE the target save file.");
    println!("Back up your saves before continuing.\n");
}

fn print_table(source_data: &[u8], target_data: &[u8]) {
    let source_chars = file_io::list_characters(source_data);
    let target_chars = file_io::list_characters(target_data);

    let mut table = Table::new();
    table.set_header(vec!["Source", "Target"]);

    for i in 0..10 {
        let s = source_chars[i]
            .as_ref()
            .map(|c| c.to_string())
            .unwrap_or_else(|| format!("Slot {i}: (Empty)"));

        let t = target_chars[i]
            .as_ref()
            .map(|c| c.to_string())
            .unwrap_or_else(|| format!("Slot {i}: (Empty)"));

        table.add_row(vec![s, t]);
    }

    println!("{table}");
}

fn ui_loop() {
    loop {
        let args = match Args::prompt() {
            Some(a) => a,
            None => break,
        };

        let source_data = match std::fs::read(&args.source) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Failed to read source file.");
                continue;
            }
        };

        let target_data = match std::fs::read(&args.target) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Failed to read target file.");
                continue;
            }
        };

        print_table(&source_data, &target_data);

        let source_slot = match prompt_int("Enter SOURCE slot (0-9)") {
            Some(v) if (0..10).contains(&v) => v,
            _ => {
                eprintln!("Invalid source slot.");
                continue;
            }
        };

        let target_slot = match prompt_int("Enter TARGET slot (0-9)") {
            Some(v) if (0..10).contains(&v) => v,
            _ => {
                eprintln!("Invalid target slot.");
                continue;
            }
        };

        let new_save = match file_io::generate_new_data(
            &source_data,
            source_slot,
            &target_data,
            target_slot,
        ) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Failed to generate save: {e}");
                continue;
            }
        };

        if let Err(e) = file_io::write_file(&new_save, &args.target.into()) {
            eprintln!("Failed to write target save: {e}");
            continue;
        }

        println!("Copy complete.\n");

        match prompt_str("Run again? (y/n)") {
            Some(ref s) if s.eq_ignore_ascii_case("y") => continue,
            _ => break,
        }
    }
}

fn main() {
    greet();
    ui_loop();
    println!("Goodbye.");
}
