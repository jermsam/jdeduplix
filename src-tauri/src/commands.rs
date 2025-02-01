use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use anyhow::Result;
use serde_json::Value;
use tracing::{info, error};
use serde::{Deserialize, Serialize};

use crate::state::{DedupManager,DuplicateGroup, DedupStrategy, SimilarityMethod, DedupResults, DedupStats};

#[derive(Debug, Deserialize)]
pub struct UpdateStrategyParams {
    pub case_sensitive: Option<bool>,
    pub ignore_whitespace: Option<bool>,
    pub ignore_punctuation: Option<bool>,
    pub normalize_unicode: Option<bool>,
    pub split_strategy: Option<String>,
    pub comparison_scope: Option<String>,
    pub min_length: Option<u32>,
    pub similarity_threshold: Option<f64>,
    pub similarity_method: Option<String>,
    pub use_parallel: Option<bool>,
}

/// Clears all texts from the deduplication manager.
#[tauri::command]
pub async fn clear(app_handle: AppHandle) -> Result<(), String> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let mut manager = state.lock().await;
    manager.clear().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_text(app_handle: AppHandle, text: String) -> Result<usize, String> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let mut manager = state.lock().await;
    Ok(manager.add_text(text))
}

#[tauri::command]
pub async fn update_strategy(app_handle: AppHandle, strategy:UpdateStrategyParams) -> Result<(), String> {
    info!("🔄 Received strategy update request");
    info!("📥 Incoming strategy data: {:#?}", strategy);
    
    // Convert the frontend strategy format to our DedupStrategy
    let similarity_method = match strategy.similarity_method.as_deref() {
        Some("Exact") => SimilarityMethod::Exact,
        Some("Semantic") => SimilarityMethod::Semantic,
        Some("Levenshtein") => SimilarityMethod::Levenshtein,
        other => {
            info!("⚠️ Unknown similarity method: {:?}, using default", other);
            SimilarityMethod::default()
        }
    };

    let similarity_threshold = strategy.similarity_threshold.unwrap_or_else(|| {
        info!("⚠️ No similarity threshold provided, using default: 1.0");
        1.0
    });

    // Get optional boolean values with defaults
    let case_sensitive = strategy.case_sensitive.unwrap_or_else(|| {
        info!("⚠️ No case_sensitive setting provided, using default: false");
        false
    });

    let ignore_punctuation = strategy.ignore_punctuation.unwrap_or_else(|| {
        info!("⚠️ No ignore_punctuation setting provided, using default: true");
        true
    });

    let ignore_whitespace = strategy.ignore_whitespace.unwrap_or_else(|| {
        info!("⚠️ No ignore_whitespace setting provided, using default: true");
        true
    });

    let normalize_unicode = strategy.normalize_unicode.unwrap_or_else(|| {
        info!("⚠️ No normalize_unicode setting provided, using default: true");
        true
    });

    let dedup_strategy = DedupStrategy {
        similarity_method,
        similarity_threshold,
        case_sensitive,
        ignore_punctuation,
        ignore_whitespace,
        normalize_unicode,
        comparison_scope: strategy.comparison_scope,
        split_strategy: strategy.split_strategy,
        min_length: strategy.min_length,
    };

    let state = app_handle.state::<Mutex<DedupManager>>();
    let mut manager = state.lock().await;
    manager.update_strategy(&serde_json::to_string(&dedup_strategy).map_err(|e| e.to_string())?).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_strategy(app_handle: AppHandle) -> Result<String, String> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let manager = state.lock().await;
    Ok(manager.get_strategy())
}

#[tauri::command]
pub async fn deduplicate_texts(app_handle: AppHandle) -> Result<DedupResults, String> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let manager = state.lock().await;
    let raw_results = manager.deduplicate_texts().map_err(|e| e.to_string())?;
    
    // Convert the raw results into our frontend-friendly format
    let mut duplicate_groups = Vec::new();
    for group in raw_results.duplicate_groups {
        // Count both original and duplicates (1 + duplicates.len())
        if group.duplicates.len() >= 1 {
            let original = group.original;
            
            let duplicates = group.duplicates;
            duplicate_groups.push(DuplicateGroup {
                original,
                duplicates,
                similarity: group.similarity,
            });
        }
    }
    
    Ok(DedupResults {
        duplicate_groups,
        stats: DedupStats {
            total_items: raw_results.stats.total_items,
            unique_items: raw_results.stats.unique_items,
            duplicate_groups: raw_results.stats.duplicate_groups,
        }
    })
}

#[tauri::command]
pub async fn get_text(app_handle: AppHandle, id: usize) -> Result<String, String> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let manager = state.lock().await;
    manager.get_text(id).ok_or_else(|| "Text not found".to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Mutex<DedupManager> {
        use crate::state::{DedupManager, DedupStrategy, SimilarityMethod};
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
            assert_eq!(results.duplicate_groups.len(), 0, "Should have no groups since there are no duplicates");
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
            assert_eq!(results.duplicate_groups.len(), 1, "Should have one group of duplicates");
            assert_eq!(results.duplicate_groups[0].duplicates.len(), 2, "Group should contain two texts");
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
        assert!(!groups.duplicate_groups.is_empty());
        
        // Check if duplicates were found
        let has_duplicates = groups.duplicate_groups.iter()
            .any(|group| group.duplicates.len() > 0);
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
            let strategy = DedupStrategy::default();
            let strategy_json = serde_json::to_string(&strategy).unwrap();
            guard.update_strategy(&strategy_json).unwrap();
        }
        
        // Verify strategy was updated
        let guard = manager.lock().await;
        let strategy = guard.get_strategy();
        assert!(strategy.contains("Exact"), "Strategy should be updated to Exact");
    }
}
