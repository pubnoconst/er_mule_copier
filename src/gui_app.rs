#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_desktop::*;

pub fn run() {
    hot_reload_init!();
    let cfg = Config::new().with_window(
        WindowBuilder::new()
        // .with_decorations(false)
        .with_window_icon(None)
        .with_title("")
        .with_min_inner_size(LogicalSize::new(800, 640))
    );
    dioxus_desktop::launch_cfg(App, cfg);
}

fn App(cx: Scope) -> Element {
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
                    "Welcome, please select the source save file",
                },
                div {
                    id: "IOCard",
                    class: "FlexContainer",
                    div {
                        id: "SourceCard",
                        class: "FlexContainer",
                        button {
                            "Browse source"
                        },
                        select {
                            option {
                                value: "slot 1",
                                "slot 1"
                            },
                            option {
                                value: "slot 2",
                                "slot 2"
                            },
                            option {
                                value: "slot 3",
                                "slot 3"
                            },
                        },
                    },
                    div {
                        id: "TargetCard",
                        class: "FlexContainer",
                        button {
                            "Browse target"
                        },
                        select {
                            option {
                                value: "slot 1",
                                "slot 1"
                            },
                            option {
                                value: "slot 2",
                                "slot 2"
                            },
                            option {
                                value: "slot 3",
                                "slot 3"
                            },
                        }
                    }
                }
                button {
                    id: "CopyButton",
                    "Copy"
                }

            }
        }    
    })
}
