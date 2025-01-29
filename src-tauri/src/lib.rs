// JDeduplix - A cutting-edge deduplication system
//! Main library entry point that coordinates all components

pub mod core;
mod commands;

use std::sync::Mutex;
use crate::core::engine::DeduplicationEngine;

type Result<T> = std::result::Result<T, String>;

/// Application state for managing deduplication
pub struct AppState {
    engine: Mutex<DeduplicationEngine>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            engine: Mutex::new(DeduplicationEngine::new()),
        }
    }
}

/// Initialize and run the Tauri application
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::process_text,
            commands::get_duplicates,
            commands::get_strategy,
            commands::update_strategy,
            commands::clear_duplicates
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
