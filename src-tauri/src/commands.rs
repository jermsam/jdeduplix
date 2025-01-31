use crate::core::classifier::TextClassifier;
use crate::core::types::{DedupStrategy, SplitStrategy, ComparisonScope, SimilarityMethod};
use crate::state::{SystemState, DedupManager, ProcessingStats};
use crate::DedupResults;
use std::sync::Arc;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;
use tauri::{Runtime, Error};
use anyhow;

static CLASSIFIER: Lazy<Arc<Mutex<TextClassifier>>> = Lazy::new(|| {
    Arc::new(Mutex::new(TextClassifier::new(DedupStrategy::default())))
});

static DEDUP_MANAGER: Lazy<Arc<DedupManager>> = Lazy::new(|| {
    Arc::new(DedupManager::new())
});

// Change from a custom Result to using Tauri's Result type
type CommandResult<T> = Result<T, tauri::Error>;

#[tauri::command]
pub async fn get_dedup_state() -> Result<SystemState, Error> {
    DEDUP_MANAGER.get_state()
        .await
        .map_err(|e| Error::from(anyhow::anyhow!(e.to_string())))
}


#[tauri::command]
pub async fn deduplicate_texts(texts: Vec<String>, strategy: DedupStrategy) -> Result<DedupResults, Error> {
    let groups = {
        let mut classifier = CLASSIFIER.lock().await;
        classifier.update_strategy(strategy);
        classifier.clear(); // Clear existing texts
        for text in &texts {
            classifier.add_text(text.clone());
        }
        let index_groups = classifier.find_duplicates();
        
        // Convert indices to actual strings
        index_groups
            .into_iter()
            .map(|group| {
                group.into_iter()
                    .map(|idx| classifier.get_text(idx).unwrap().clone())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>()
    };

    Ok(DedupResults {
        groups,
        stats: ProcessingStats {
            total_items: texts.len(),
            processing_time: 0.0,  // You might want to actually measure this
            memory_used: 0,        // You might want to actually measure this
        }
    })
}

#[tauri::command]
pub async fn add_text(text: String) -> Result<usize, Error> {
    let mut classifier = CLASSIFIER.lock().await;
    Ok(classifier.add_text(text))
}

#[tauri::command]
pub async fn find_duplicates() -> Result<Vec<Vec<String>>, Error> {
    let classifier = CLASSIFIER.lock().await;
    let index_groups = classifier.find_duplicates();
    let all_texts = classifier.get_all_texts();
    
    // Convert indices to actual strings
    Ok(index_groups
        .into_iter()
        .map(|group| {
            group.into_iter()
                .map(|idx| all_texts[idx].clone())
                .collect()
        })
        .collect())
}

#[tauri::command]
pub async fn get_text(idx: usize) -> Result<Option<String>, Error> {
    let classifier = CLASSIFIER.lock().await;
    Ok(classifier.get_text(idx).cloned())
}

#[tauri::command]
pub async fn get_all_texts() -> Result<Vec<String>, Error> {
    let classifier = CLASSIFIER.lock().await;
    Ok(classifier.get_all_texts())
}

#[tauri::command]
pub async fn clear() -> Result<(), Error> {
    let mut classifier = CLASSIFIER.lock().await;
    classifier.clear();
    Ok(())
}

#[tauri::command]
pub async fn update_strategy(
    similarity_threshold: f64,
    case_sensitive: bool,
    ignore_whitespace: bool,
    ignore_punctuation: bool,
    normalize_unicode: bool,
    split_strategy: SplitStrategy,
    comparison_scope: ComparisonScope,
    min_length: usize,
    similarity_method: SimilarityMethod,
    use_parallel: bool,
) -> Result<(), Error> {
    let mut classifier = CLASSIFIER.lock().await;
    classifier.update_strategy(DedupStrategy {
        similarity_threshold,
        case_sensitive,
        ignore_whitespace,
        ignore_punctuation,
        normalize_unicode,
        split_strategy,
        comparison_scope,
        min_length,
        similarity_method,
        use_parallel,
    });
    Ok(())
}

#[tauri::command]
pub async fn get_strategy() -> Result<DedupStrategy, Error> {
    let classifier = CLASSIFIER.lock().await;
    Ok(classifier.get_strategy().clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    // Helper function to test the functionality directly without Tauri's State
    async fn test_with_classifier<F, Fut>(f: F) -> Fut::Output
    where
        F: FnOnce(TextClassifier) -> Fut,
        Fut: std::future::Future,
    {
        let classifier = TextClassifier::default();
        f(classifier).await
    }

    #[tokio::test]
    async fn test_add_and_get_text() {
        test_with_classifier(|mut classifier| async move {
            let text = "Hello, World!".to_string();
            
            // Test add_text
            let idx = classifier.add_text(text.clone());
            
            // Test get_text
            let retrieved = classifier.get_text(idx);
            assert_eq!(retrieved, Some(&text));
            
            // Test get_all_texts
            let all_texts = classifier.get_all_texts();
            assert_eq!(all_texts, vec![text]);
        }).await;
    }

    #[tokio::test]
    async fn test_find_duplicates() {
        test_with_classifier(|mut classifier| async move {
            // Set appropriate strategy for finding similar texts
            classifier.update_strategy(DedupStrategy {
                case_sensitive: false,
                ignore_whitespace: true,
                ignore_punctuation: true,
                normalize_unicode: true,
                split_strategy: SplitStrategy::WholeText,
                comparison_scope: ComparisonScope::Global,
                min_length: 5,
                similarity_threshold: 0.6, // More lenient threshold
                similarity_method: SimilarityMethod::Semantic,
                use_parallel: false,
            });

            // Add some test texts with more similar wording
            let idx1 = classifier.add_text("The cat quickly jumped over the fence".to_string());
            let idx2 = classifier.add_text("The cat swiftly jumped over the fence".to_string());
            let idx3 = classifier.add_text("A dog runs in the park".to_string());
            let idx4 = classifier.add_text("My dog sleeps on the couch".to_string());
            
            let duplicates = classifier.find_duplicates();
            
            // Should find at least one group of similar texts
            assert!(!duplicates.is_empty());
            
            // Find the group containing cat-related texts
            let cat_group = duplicates.iter()
                .find(|group| group.iter().any(|&id| id == idx1))
                .expect("Should find cat-related group");
            
            // Verify that similar cat texts are in the same group
            assert!(cat_group.iter().any(|&id| id == idx2));
            
            // Verify that dissimilar texts are not in the cat group
            assert!(!cat_group.iter().any(|&id| id == idx3));
            assert!(!cat_group.iter().any(|&id| id == idx4));
        }).await;
    }

    #[tokio::test]
    async fn test_update_and_get_strategy() {
        test_with_classifier(|mut classifier| async move {
            let strategy = DedupStrategy {
                similarity_threshold: 0.8,
                case_sensitive: false,
                ignore_whitespace: true,
                ignore_punctuation: true,
                normalize_unicode: true,
                split_strategy: SplitStrategy::default(),
                comparison_scope: ComparisonScope::default(),
                min_length: 5,
                similarity_method: SimilarityMethod::default(),
                use_parallel: true,
            };
            
            // Update strategy
            classifier.update_strategy(strategy.clone());
            
            // Get and verify strategy
            let retrieved_strategy = classifier.get_strategy();
            assert_eq!(retrieved_strategy.similarity_threshold, strategy.similarity_threshold);
            assert_eq!(retrieved_strategy.case_sensitive, strategy.case_sensitive);
            assert_eq!(retrieved_strategy.ignore_whitespace, strategy.ignore_whitespace);
            assert_eq!(retrieved_strategy.ignore_punctuation, strategy.ignore_punctuation);
            assert_eq!(retrieved_strategy.normalize_unicode, strategy.normalize_unicode);
            assert_eq!(retrieved_strategy.split_strategy, strategy.split_strategy);
            assert_eq!(retrieved_strategy.comparison_scope, strategy.comparison_scope);
            assert_eq!(retrieved_strategy.min_length, strategy.min_length);
            assert_eq!(retrieved_strategy.similarity_method, strategy.similarity_method);
            assert_eq!(retrieved_strategy.use_parallel, strategy.use_parallel);
        }).await;
    }

    #[tokio::test]
    async fn test_clear() {
        test_with_classifier(|mut classifier| async move {
            // Add some texts
            classifier.add_text("Text 1".to_string());
            classifier.add_text("Text 2".to_string());
            
            // Verify texts were added
            assert_eq!(classifier.get_all_texts().len(), 2);
            
            // Clear texts
            classifier.clear();
            
            // Verify texts were cleared
            assert_eq!(classifier.get_all_texts().len(), 0);
        }).await;
    }
}
