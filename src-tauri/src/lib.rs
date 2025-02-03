// JDeduplix - A cutting-edge deduplication system
//! Main library entry point that coordinates all component
pub mod commands;
pub mod core;
pub mod state;
pub mod config;
pub use state::*;
pub mod presets;

use crate::state::{DedupManager, DedupStrategySettings, SimilarityMethod};
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(DedupManager::new(
            DedupStrategySettings::default(),
            SimilarityMethod::default(),
        )))
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::add_text,
            commands::get_text,
            commands::clear,
            commands::update_strategy,
            commands::get_strategy,
            commands::deduplicate_texts,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
