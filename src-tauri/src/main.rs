#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use clap::{
    arg,
    command,
};
use log::*;

struct TargetFile(Option<String>);

#[tauri::command]
fn gethtml(file: tauri::State<TargetFile>) -> String {
    debug!("updating?");

    let markdown = if let Some(file) = &file.0 {
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
    
    let file = if let Some(filename) = matches.get_one::<String>("FILE") {
        Some(filename.into())
    } else {
        info!("TODO: actually support launching without a file");

        #[cfg(not(debug_assertions))]
        { None }

        #[cfg(debug_assertions)]
        Some("../TEST.md".into())
    };
    
    
    tauri::Builder::default()
        .manage(TargetFile(file))
        .invoke_handler(tauri::generate_handler![gethtml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
