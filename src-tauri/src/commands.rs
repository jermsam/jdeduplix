use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;
use tracing::info;
use crate::config::DynamicConfig;
use crate::error::{AppError, ErrorCode};
use crate::state::{DedupManager,DuplicateGroup, DedupStrategySettings,  DedupResults, DedupStats};

/// Clears all texts from the deduplication manager.
#[tauri::command]
pub async fn clear(app_handle: AppHandle) -> Result<(), AppError> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let mut manager = state.lock().await;
    manager.clear().map_err(|e| AppError::new(ErrorCode::InternalError, e.to_string()))
}

#[tauri::command]
pub async fn add_text(app_handle: AppHandle, text: String) -> Result<usize, AppError> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let mut manager = state.lock().await;
    Ok(manager.add_text(text))
}

#[tauri::command]
pub async fn update_strategy(app_handle: AppHandle, strategy: String) -> Result<String, AppError> {
    info!("🔄 Received strategy update request");
    let strategy: DedupStrategySettings = serde_json::from_str(&strategy).map_err(|e| AppError::new(ErrorCode::DeserializationError, e.to_string()))?;
    info!("📥 Incoming strategy data: {:#?}",strategy);

    let dedup_strategy = DedupStrategySettings {
        case_sensitive: strategy.case_sensitive,
        ignore_whitespace: strategy.ignore_whitespace,
        ignore_punctuation: strategy.ignore_punctuation,
        normalize_unicode: strategy.normalize_unicode,
        split_strategy: strategy.split_strategy,
        comparison_scope: strategy.comparison_scope,
        min_length: strategy.min_length,
        similarity_threshold: strategy.similarity_threshold,
        max_duplicate_count: strategy.max_duplicate_count,
        similarity_method: strategy.similarity_method,
        use_parallel: strategy.use_parallel,
        ignore_stopwords: strategy.ignore_stopwords,
        stemming: strategy.stemming,
        ngram_size: strategy.ngram_size,
        language_detection: Some(false),
        encoding_normalization: Some(true),
        adaptive_thresholding: Some(false),
        config: Some(DynamicConfig::default()),
    };

    let state = app_handle.state::<Mutex<DedupManager>>();
    let mut manager = state.lock().await;
    
    // Update the strategy and return the updated strategy as JSON
    let updated_strategy_str = serde_json::to_string(&dedup_strategy)
        .map_err(|e| AppError::new(
            ErrorCode::SerializationError,
            format!("Failed to serialize strategy: {}", e)
        ))?;
    
    manager.update_strategy(&updated_strategy_str)
        .map_err(|e| AppError::new(
            ErrorCode::StrategyUpdateError,
            format!("Failed to update strategy: {}", e)
        ))?;
    
    Ok(updated_strategy_str)
}

#[tauri::command]
pub async fn get_strategy(app_handle: AppHandle) -> Result<DedupStrategySettings, AppError> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let manager = state.lock().await;
    let strategy_str = manager.get_strategy();
    serde_json::from_str(&strategy_str).map_err(|e| AppError::new(ErrorCode::DeserializationError, e.to_string()))
}

#[tauri::command]
pub async fn deduplicate_texts(app_handle: AppHandle) -> Result<DedupResults, AppError> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let mut manager = state.lock().await;
    let raw_results = manager.deduplicate_texts().map_err(|e| AppError::new(ErrorCode::InternalError, e.to_string()))?;
    
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
pub async fn get_text(app_handle: AppHandle, id: usize) -> Result<String, AppError> {
    let state = app_handle.state::<Mutex<DedupManager>>();
    let manager = state.lock().await;
    manager.get_text(id).ok_or_else(|| AppError::new(ErrorCode::InternalError, "Text not found".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Mutex<DedupManager> {
        use crate::state::{DedupManager, DedupStrategySettings, SimilarityMethod};
        Mutex::new(DedupManager::new(
            DedupStrategySettings::default(),
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
        let mut guard = manager.lock().await;
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
            let mut guard = manager.lock().await;
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
            let mut guard = manager.lock().await;
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
            let strategy = DedupStrategySettings::default();
            let strategy_json = serde_json::to_string(&strategy).unwrap();
            guard.update_strategy(&strategy_json).unwrap();
        }
        
        // Verify strategy was updated
        let guard = manager.lock().await;
        let strategy = guard.get_strategy();
        assert!(strategy.contains("Exact"), "Strategy should be updated to Exact");
    }
}
