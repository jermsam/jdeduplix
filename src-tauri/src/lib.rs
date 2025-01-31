// JDeduplix - A cutting-edge deduplication system
//! Main library entry point that coordinates all components

mod commands;
mod core;
mod state;

use std::sync::Arc;
use crate::core::types::DedupStrategy;
use state::{DedupManager, SystemState, DedupResults};
use once_cell::sync::Lazy;

type Result<T> = std::result::Result<T, String>;

static DEDUP_MANAGER: Lazy<Arc<DedupManager>> = Lazy::new(|| {
    Arc::new(DedupManager::new())
});

// Re-export commands for Tauri
pub use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::add_text,
            commands::find_duplicates,
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
