use tauri::State;
use crate::core::vector::DedupStrategy;
use crate::AppState;
use crate::Result;

/// Tauri command to process text and check for duplicates
#[tauri::command]
pub fn process_text(state: State<AppState>, content: String) -> Result<usize> {
    println!("Command: process_text called with content length: {}", content.len());
    let result = state.engine
        .lock()
        .map_err(|e| e.to_string())?
        .process_text(content);
    println!("Command: process_text returned index: {}", result);
    Ok(result)
}

/// Tauri command to retrieve all duplicate groups
#[tauri::command]
pub fn get_duplicates(state: State<AppState>) -> Result<Vec<Vec<String>>> {
    println!("Command: get_duplicates called");
    let dupes: Vec<Vec<String>> = state.engine
        .lock()
        .map_err(|e| e.to_string())?
        .get_duplicates()
        .into_iter()
        .map(|group| group.into_iter().map(String::from).collect())
        .collect();
    println!("Command: get_duplicates returning {} groups", dupes.len());
    Ok(dupes)
}

/// Tauri command to get current deduplication strategy
#[tauri::command]
pub fn get_strategy(state: State<AppState>) -> Result<DedupStrategy> {
    println!("Command: get_strategy called");
    let strategy = state.engine
        .lock()
        .map_err(|e| e.to_string())?
        .get_strategy();
    println!("Command: get_strategy returning {:?}", strategy);
    Ok(strategy)
}

/// Tauri command to update deduplication strategy
#[tauri::command]
pub fn update_strategy(state: State<AppState>, strategy: DedupStrategy) -> Result<()> {
    println!("Command: update_strategy called with {:?}", strategy);
    state.engine
        .lock()
        .map_err(|e| e.to_string())?
        .update_strategy(strategy);
    println!("Command: update_strategy completed");
    Ok(())
}

/// Tauri command to clear all duplicate groups
#[tauri::command]
pub fn clear_duplicates(state: State<AppState>) -> Result<()> {
    println!("Command: clear_duplicates called");
    state.engine
        .lock()
        .map_err(|e| e.to_string())?
        .clear_duplicates();
    println!("Command: clear_duplicates completed");
    Ok(())
}
