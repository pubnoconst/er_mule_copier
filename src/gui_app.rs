#![allow(non_snake_case)]
use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus_desktop::*;

use crate::{helpers, save_model, file_io};

pub fn run() {
    let cfg = Config::new().with_window(
        WindowBuilder::new()
        .with_title("")
        .with_min_inner_size(LogicalSize::new(800, 670))
    );
    dioxus_desktop::launch_cfg(App, cfg);
}


fn App(cx: Scope) -> Element {
    let banner = use_state(cx, || String::from("Welcome, please select the source and target save file"));
    let input_filename = use_state(cx, || Option::<PathBuf>::None);
    let input_game_data = use_ref(cx, || Vec::<u8>::with_capacity(30 * 1024));
    let target_game_data = use_ref(cx, || Vec::<u8>::with_capacity(30 * 1024));
    let target_filename = use_state(cx, || Option::<PathBuf>::None);
    let input_slots = use_state(cx, || Vec::<save_model::Save>::new());
    let target_slots = use_state(cx, || Vec::<save_model::Save>::new());
    let input_save_slot = use_state(cx, || Option::<usize>::None);
    let target_save_slot = use_state(cx, || Option::<usize>::None);


    cx.render(rsx! {
        style { include_str!("style.css") },
        div { 
            id: "content",
            div {
                id: "Headerbar",
                p {
                    id: "Title",
                    "ER Mule Copier"
                },
                button {
                    id: "ExitButton",
                    onclick: |_| std::process::exit(0),
                    "Exit"
                }
            }
            div {
                id: "MainCard",
                class: "FlexContainer",
                p {
                    id: "Guide",
                    class: "FlexContainer",
                    banner.as_str()
                },
                div {
                    id: "IOCard",
                    class: "FlexContainer",
                    div {
                        id: "SourceCard",
                        class: "FlexContainer",
                        p {
                            class: "CardTitle",
                            u {
                                "Source"
                            }
                        },
                        button {
                            onclick: move |_| {
                                let file = rfd::FileDialog::new().add_filter(".sl2", &["sl2"]).pick_file();
                                input_filename.set(file.clone());
                                if let Some(f) =  file {
                                    let game_data = std::fs::read(f).unwrap();
                                    input_slots.set(file_io::list_saves(&game_data));
                                    input_game_data.set(game_data);                                    
                                    input_save_slot.set(Some(0));
                                } else {
                                    input_slots.set(Vec::new());
                                    input_save_slot.set(None);
                                    input_game_data.set(Vec::with_capacity(30 * 1024));
                                }
                            },
                            if let Some(pb) = input_filename.get() {
                                helpers::truncate_path(pb)
                            } else {
                                format!("Browse source")
                            }
                        },
                        select {
                            id: "SourceSelection",
                            onchange: move |selection_event| {
                                input_save_slot.set(selection_event.data.value.parse().ok());
                            },
                            
                            for save in input_slots.get() {
                                option {
                                    value: "{save.slot_index}",
                                    "{save.to_string()}"
                                }
                            }
                        },
                    },
                    div {
                        id: "TargetCard",
                        class: "FlexContainer",
                        p {
                            class: "CardTitle",
                            u {
                                "Target"
                            }
                        },
                        button {
                            onclick: move |_| {
                                let file = rfd::FileDialog::new().add_filter(".sl2", &["sl2"]).pick_file();
                                target_filename.set(file.clone());
                                if let Some(f) =  file {
                                    let game_data = std::fs::read(f).unwrap();
                                    target_slots.set(file_io::list_saves(&game_data));
                                    target_game_data.set(game_data);
                                    target_save_slot.set(Some(0));
                                } else {
                                    target_slots.set(Vec::new());
                                    target_save_slot.set(None);
                                    target_game_data.set(Vec::with_capacity(30 * 1024));
                                }
                            },
                            if let Some(pb) = target_filename.get() {
                                helpers::truncate_path(pb)
                            } else {
                                format!("Browse target")
                            }
                        },
                        select {
                            id: "TargetSelection",
                            onchange: move |selection_event| {
                                target_save_slot.set(selection_event.data.value.parse().ok());
                            },
                            for save in target_slots.get() {
                                option {
                                    value: "{save.slot_index}",
                                    "{save.to_string()}"
                                }
                            }
                        }
                    }
                }
                
                button {
                    id: "CopyButton",
                    onclick: move |evt| {
                        match (input_filename.get(), target_filename.get(), input_save_slot.get(), target_save_slot.get()) {
                            (Some(i_f), Some(t_f), Some(i_s), Some(t_s)) 
                            if 
                                input_game_data.read().len() > 0 
                                && input_slots.len() > 0  
                                && target_slots.len() > 0
                                && target_game_data.read().len() > 0
                            => {
                                // backup
                                let backup_file_name = format!(
                                    "{}.BAK{}", 
                                    t_f
                                        .as_path()
                                        .display(), 
                                    helpers::get_unix_timestamp());
                                std::fs::write(&backup_file_name, &*target_game_data.read()).expect("Could not write backup");
                                println!("Backup stored as: {}", backup_file_name);

                                    // get generated content
                                    let generated_save_data = file_io::generate_new_save_file_content(
                                    &*input_game_data.read(),
                                    &input_slots[*i_s],
                                    &*target_game_data.read(),
                                    &target_slots[*t_s]
                                );
                                // write
                                std::fs::write(t_f, generated_save_data).expect("Could not write save file");
                                // Indicate
                                banner.set(format!(
                                                     "{} has been overwritten with {}", 
                                                     &target_slots[*t_s],
                                                     &input_slots[*i_s]
                                           )
                                );

                                // Reload the files
                                let game_data = std::fs::read(i_f).unwrap();
                                input_slots.set(file_io::list_saves(&game_data));
                                input_game_data.set(game_data);                                    

                                let game_data = std::fs::read(t_f).unwrap();
                                target_slots.set(file_io::list_saves(&game_data));
                                target_game_data.set(game_data);    
                            },
                            _ => {
                                banner
                                    .set("Sorry, something went wrong, please provide character slots as required".into());
                            }
                        }
                        evt.stop_propagation();                       
                    },
                    "Copy",
                }

            }
        }    
    })
}
