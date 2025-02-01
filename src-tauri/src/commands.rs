use tauri::State;
use tokio::sync::Mutex;
use anyhow::Result;

use crate::state::{DedupManager, DedupResults};
use crate::core::types::{DedupStrategy, SimilarityMethod};

/// Clears all texts from the deduplication manager.
#[tauri::command]
pub async fn clear(state: State<'_, Mutex<DedupManager>>) -> Result<(), String> {
    let mut manager = state.lock().await;
    manager.clear().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_text(state: State<'_, Mutex<DedupManager>>, text: String) -> Result<usize, String> {
    let mut manager = state.lock().await;
    Ok(manager.add_text(text))
}

#[tauri::command]
pub async fn update_strategy(state: State<'_, Mutex<DedupManager>>, strategy: String) -> Result<(), String> {
    let mut manager = state.lock().await;
    manager.update_strategy(&strategy).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_strategy(state: State<'_, Mutex<DedupManager>>) -> Result<String, String> {
    let manager = state.lock().await;
    Ok(manager.get_strategy())
}

#[tauri::command]
pub async fn deduplicate_texts(state: State<'_, Mutex<DedupManager>>) -> Result<DedupResults, String> {
    let manager = state.lock().await;
    manager.deduplicate_texts().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_text(state: State<'_, Mutex<DedupManager>>, id: usize) -> Result<String, String> {
    let manager = state.lock().await;
    manager.get_text(id).ok_or_else(|| "Text not found".to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Mutex<DedupManager> {
        use crate::core::types::{DedupStrategy, SimilarityMethod};
        Mutex::new(DedupManager::new(
            DedupStrategy::default(),
            SimilarityMethod::default(),
        ))
    }

    #[tokio::test]
    async fn test_add_text() {
        let manager = setup();
        
        // Test adding a single text
        {
            let mut guard = manager.lock().await;
            let text = "Hello world".to_string();
            let index = guard.add_text(text);
            assert_eq!(index, 0, "First added text should have index 0");
            
            // Verify the text was stored correctly by checking deduplication results
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.groups.len(), 0, "Should have no groups since there are no duplicates");
        }
        
        // Test adding multiple texts
        {
            let mut guard = manager.lock().await;
            let index = guard.add_text("Another text".to_string());
            assert_eq!(index, 1, "Second added text should have index 1");
            
            // Verify both texts are stored
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.stats.total_items, 2, "Should have two texts stored");
        }
        
        // Add a duplicate text to create a group
        {
            let mut guard = manager.lock().await;
            guard.add_text("Hello world".to_string());
            
            // Now we should have one group with the duplicates
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.groups.len(), 1, "Should have one group of duplicates");
            assert_eq!(results.groups[0].len(), 2, "Group should contain two texts");
        }
    }

    #[tokio::test]
    async fn test_find_duplicates() {
        let manager = setup();
        
        // Add some duplicate texts
        {
            let mut guard = manager.lock().await;
            guard.add_text("Hello world".to_string());
            guard.add_text("Different text".to_string());
            guard.add_text("Hello world".to_string()); // Duplicate
        }
        
        // Test deduplication
        let guard = manager.lock().await;
        let groups = guard.deduplicate_texts().unwrap();
        assert!(!groups.groups.is_empty());
        
        // Check if duplicates were found
        let has_duplicates = groups.groups.iter()
            .any(|group| group.len() > 1);
        assert!(has_duplicates);
    }

    #[tokio::test]
    async fn test_clear_texts() {
        let manager = setup();
        
        // Add texts
        {
            let mut guard = manager.lock().await;
            guard.add_text("Text 1".to_string());
            guard.add_text("Text 2".to_string());
        }
        
        // Verify texts were added
        {
            let guard = manager.lock().await;
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.stats.total_items, 2, "Expected 2 texts to be added");
        }
        
        // Test clearing
        {
            let mut guard = manager.lock().await;
            guard.clear().unwrap();
        }
        
        // Verify texts were cleared
        {
            let guard = manager.lock().await;
            let results = guard.deduplicate_texts().unwrap();
            assert_eq!(results.stats.total_items, 0, "Expected all texts to be cleared");
        }
    }

    #[tokio::test]
    async fn test_update_strategy() {
        let manager = setup();
        
        // Test updating strategy
        {
            let mut guard = manager.lock().await;
            guard.update_strategy(r#"{
                "similarity_method": "Exact",
                "similarity_threshold": 1.0
            }"#).unwrap();
        }
        
        // Verify strategy was updated
        let guard = manager.lock().await;
        let strategy = guard.get_strategy();
        assert!(strategy.contains("Exact"), "Strategy should be updated to Exact");
    }
}
