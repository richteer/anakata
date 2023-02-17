#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::sync::Mutex;
use clap::{
    arg,
    command,
};
use lazy_static::lazy_static;
use log::*;

struct AnakataState {
    file: Option<String>,
}

impl Default for AnakataState {
    fn default() -> Self {
        Self {
            file: None
        }
    }
}

// TODO: figure out if there is a better way to store this information "globally"
// TODO: use inotify or something instead so it refreshes when it needs to
lazy_static! {
    static ref STATE: Mutex<AnakataState> = Mutex::new(AnakataState::default());
}

#[tauri::command]
fn gethtml() -> String {
    debug!("updating?");

    let state = STATE.lock().unwrap();
    let markdown = if let Some(file) = &state.file {
        fs::read_to_string(file).unwrap_or_else(|_| format!(" # File '{}' not found!", file))
    } else {
        "# File not found!".to_string()
    };

    markdown::to_html_with_options(markdown.as_str(), &markdown::Options::gfm())
        .unwrap_or_else(|e| format!("Markdown failed to parse: {:?}", e))
}

fn main() {
    env_logger::builder()
        .filter(None, LevelFilter::Debug)
        .init();

    let matches = command!()
        .arg(arg!(<FILE>).required(false))
        .get_matches()
    ;

    #[cfg(debug_assertions)]
    {
        let mut state = STATE.lock().unwrap();
        state.file = Some("../README.md".to_string());
    }
    
    if let Some(file) = matches.get_one::<String>("FILE") {
        let mut state = STATE.lock().unwrap();
        state.file = Some(file.to_string())
    } else {
        info!("TODO: actually support launching without a file")
    }
    
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![gethtml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
