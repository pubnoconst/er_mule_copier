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
    let output_filename = use_state(cx, || Option::<PathBuf>::None);
    let input_slots = use_state(cx, || Vec::<save_model::Save>::new());
    let output_slots = use_state(cx, || Vec::<save_model::Save>::new());
    let input_save_slot = use_state(cx, || Option::<usize>::None);
    let output_save_slot = use_state(cx, || Option::<usize>::None);


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
                                    input_slots.set(file_io::list_saves(&std::fs::read(f).unwrap()));
                                    input_save_slot.set(Some(0));
                                } else {
                                    input_slots.set(Vec::new());
                                    input_save_slot.set(None);
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
                                output_filename.set(file.clone());
                                if let Some(f) =  file {
                                    output_slots.set(file_io::list_saves(&std::fs::read(f).unwrap()));
                                    output_save_slot.set(Some(0));
                                } else {
                                    output_slots.set(Vec::new());
                                    output_save_slot.set(None);
                                }
                            },
                            if let Some(pb) = output_filename.get() {
                                helpers::truncate_path(pb)
                            } else {
                                format!("Browse target")
                            }
                        },
                        select {
                            id: "TargetSelection",
                            onchange: move |selection_event| {
                                output_save_slot.set(selection_event.data.value.parse().ok());
                            },
                            for save in output_slots.get() {
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
                        println!("Selected source file: {:?}\nSelected Slot: {:?}\n", input_filename, input_save_slot);
                        println!("Selected target file: {:?}\nSelected Slot: {:?}\n", output_filename, output_save_slot);
                        evt.stop_propagation();
                    },
                    "Copy",
                }

            }
        }    
    })
}
