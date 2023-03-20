use cli_table::{format::Justify, Cell, Style, Table};
use gumdrop::Options;
use text_io;
use crate::{file_io, helpers};

#[derive(Options)]
struct Args {
    #[options(help = "Input save file")]
    input: String,

    #[options(help = "Output save file")]
    output: String,
}

pub fn run() {
    let opts = Args::parse_args_default_or_exit();
    let input_file_content = std::fs::read(&opts.input).unwrap();
    let target_file_content = std::fs::read(&opts.output).unwrap();

    let input_saves = file_io::list_saves(&input_file_content);
    let target_saves = file_io::list_saves(&target_file_content);

    // display table
    let table_vec = {
        let mut table_vec = Vec::with_capacity(10);
        for i in 1..11 {
            table_vec.push(vec![
                input_saves[i - 1]
                    .to_string()
                    .cell()
                    .justify(Justify::Right),
                target_saves[i - 1]
                    .to_string()
                    .cell()
                    .justify(Justify::Left),
            ]);
        }
        table_vec
    };
    let table = table_vec
        .table()
        .title(vec![
            "Mule file".cell().justify(Justify::Right).bold(true),
            "Your save file".cell().justify(Justify::Left).bold(true),
        ])
        .bold(true);

    let table_display = table.display().unwrap();
    println!("{}", table_display);

    // get slots
    println!("Enter source save slot, followed by the target save slot, separated by an empty space: (i.e. \"2 5\")");
    let from_slot: usize;
    let to_slot: usize;
    text_io::scan!("{} {}", from_slot, to_slot);
    assert!((1..11).contains(&from_slot) && (1..11).contains(&to_slot));
    
    print!("Copying character from mule slot {} to save slot {}...", from_slot, to_slot);


    // backup
    let backup_file_name = format!("{}.BAK{}", opts.output, helpers::get_unix_timestamp());
    std::fs::write(backup_file_name, &target_file_content).expect("Could not write backup");

    // write
    let source_save = crate::save_model::Save::from_raw_data(from_slot, &input_file_content);
    let target_save = &crate::save_model::Save::from_raw_data(to_slot, &target_file_content);
    let genareted_save = file_io::generate_new_save_file_content(&input_file_content, &source_save, &target_file_content, target_save);
    std::fs::write(opts.output, genareted_save).unwrap();

    // confirm
    println!("Success. Exiting.");
}

