// JDeduplix - A cutting-edge deduplication system
//! Main library entry point that coordinates all component
pub mod core;
mod commands;
mod state;

use std::sync::{Arc, Mutex};
use state::DedupManager;
use once_cell::sync::Lazy;
use crate::core::types::{DedupStrategy, SimilarityMethod};

static DEDUP_MANAGER: Lazy<Arc<Mutex<DedupManager>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DedupManager::new(DedupStrategy {
        similarity_method: SimilarityMethod::Semantic,
        ..DedupStrategy::default()
    })))
});

// Re-export commands for Tauri
pub use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DEDUP_MANAGER.clone())  // Register the Mutex-wrapped DedupManager
        .invoke_handler(tauri::generate_handler![
            commands::add_text,
            commands::get_text,
            commands::clear,
            commands::update_strategy,
            commands::get_strategy,
            commands::deduplicate_texts,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
