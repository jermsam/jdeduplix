// JDeduplix - A cutting-edge deduplication system
//! Main library entry point that coordinates all components

pub mod core {
    pub mod vector;
    pub mod classifier;
    pub mod types;
    pub mod semantic;
}
pub mod settings;
pub mod commands;

use std::sync::{Arc, Mutex};
use crate::core::classifier::TextClassifier;
use crate::core::types::DedupStrategy;

type Result<T> = std::result::Result<T, String>;

#[derive(Default, Clone)]
pub struct AppState {
    pub classifier: Arc<Mutex<TextClassifier>>,
}

/// Initialize and run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(AppState {
            classifier: Arc::new(Mutex::new(TextClassifier::new(DedupStrategy::default())))
        })
        .invoke_handler(tauri::generate_handler![
            commands::add_text,
            commands::find_duplicates,
            commands::get_text,
            commands::get_all_texts,
            commands::clear,
            commands::update_strategy,
            commands::get_strategy,
        ])
        .build(context)
        .expect("error while building tauri application")
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                api.prevent_exit();
            }
        });
}
