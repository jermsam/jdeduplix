use tauri::State;
use std::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;
use anyhow::Result;

use crate::state::{DedupManager, DedupResults};

/// Clears all texts from the deduplication manager.
#[tauri::command]
pub fn clear(state: State<'_, Mutex<DedupManager>>) -> Result<(), String> {
    let manager = state.lock().unwrap();
    manager.clear().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_text(state: State<'_, Mutex<DedupManager>>, text: String) -> Result<usize, String> {
    let manager = state.lock().unwrap();
    Ok(manager.add_text(text))
}

#[tauri::command]
pub fn update_strategy(state: State<'_, Mutex<DedupManager>>, strategy: String) -> Result<(), String> {
    let manager = state.lock().unwrap();
    manager.update_strategy(&strategy).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_strategy(state: State<'_, Mutex<DedupManager>>) -> Result<String, String> {
    let manager = state.lock().unwrap();
    Ok(manager.get_strategy())
}

#[tauri::command]
pub fn deduplicate_texts(state: State<'_, Mutex<DedupManager>>) -> Result<DedupResults, String> {
    let manager = state.lock().unwrap();
    manager.deduplicate_texts().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_text(state: State<'_, AsyncMutex<DedupManager>>, id: usize) -> Result<String, String> {
    let manager = state.lock().await;
    manager.get_text(id).ok_or_else(|| "Text not found".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Mutex<DedupManager> {
        use crate::core::types::DedupStrategy;
        Mutex::new(DedupManager::new(DedupStrategy::default()))
    }

    #[test]
    fn test_add_text() {
        let manager = setup();
        
        // Test adding a single text
        {
            let guard = manager.lock().unwrap();
            let index = guard.add_text("Hello world".to_string());
            assert_eq!(index, 0, "First added text should have index 0");
            
            // Verify the text was stored correctly by checking deduplication results
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.groups.len(), 0, "Should have no groups since there are no duplicates");
        }
        
        // Test adding multiple texts
        {
            let guard = manager.lock().unwrap();
            let index = guard.add_text("Another text".to_string());
            assert_eq!(index, 1, "Second added text should have index 1");
            
            // Verify both texts are present using deduplication results
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.groups.len(), 0, "Should have no groups since there are no duplicates");
        }
        
        // Add a duplicate text to create a group
        {
            let guard = manager.lock().unwrap();
            guard.add_text("Hello world".to_string());
            
            // Now we should have one group with the duplicates
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.groups.len(), 1, "Should have one group for duplicates");
            assert_eq!(results.groups[0].len(), 2, "Group should contain both instances of 'Hello world'");
            assert!(results.groups[0].contains(&"Hello world".to_string()), "Should contain the duplicate text");
        }
    }

    #[test]
    fn test_find_duplicates() {
        let manager = setup();
        
        // Add some duplicate texts
        {
            let guard = manager.lock().unwrap();
            guard.add_text("Hello world".to_string());
            guard.add_text("Different text".to_string());
            guard.add_text("Hello world".to_string()); // Duplicate
        }
        
        // Test deduplication
        let guard = manager.lock().unwrap();
        let groups = guard.deduplicate_texts().unwrap();
        assert!(!groups.groups.is_empty());
        
        // Check if duplicates were found
        let has_duplicates = groups.groups.iter()
            .any(|group| group.len() > 1);
        assert!(has_duplicates);
    }

    #[test]
    fn test_clear_texts() {
        let manager = setup();
        
        // Add texts
        {
            let guard = manager.lock().unwrap();
            guard.add_text("Text 1".to_string());
            guard.add_text("Text 2".to_string());
        }
        
        // Verify texts were added
        {
            let guard = manager.lock().unwrap();
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.stats.total_items, 2, "Expected 2 texts to be added");
        }
        
        // Test clearing
        {
            let guard = manager.lock().unwrap();
            guard.clear().unwrap();
        }
        
        // Verify texts were cleared
        {
            let guard = manager.lock().unwrap();
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.stats.total_items, 0, "Expected all texts to be cleared");
        }
    }

    #[test]
    fn test_update_strategy() {
        let manager = setup();
        
        // Test updating strategy
        {
            let guard = manager.lock().unwrap();
            guard.update_strategy(r#"{
                "similarity_method": "Exact",
                "similarity_threshold": 1.0,
                "case_sensitive": true,
                "ignore_whitespace": false,
                "ignore_punctuation": false,
                "normalize_unicode": true,
                "split_strategy": "WholeText",
                "comparison_scope": "Global",
                "min_length": 1,
                "use_parallel": false
            }"#).unwrap();
        }
        
        // Verify strategy was updated by parsing and re-serializing both JSONs
        let guard = manager.lock().unwrap();
        let actual_json: serde_json::Value = serde_json::from_str(&guard.get_strategy()).unwrap();
        let expected_json: serde_json::Value = serde_json::from_str(r#"{
            "similarity_method": "Exact",
            "similarity_threshold": 1.0,
            "case_sensitive": true,
            "ignore_whitespace": false,
            "ignore_punctuation": false,
            "normalize_unicode": true,
            "split_strategy": "WholeText",
            "comparison_scope": "Global",
            "min_length": 1,
            "use_parallel": false
        }"#).unwrap();
        assert_eq!(actual_json, expected_json);
    }
}
