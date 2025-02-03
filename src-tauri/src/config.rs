use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use stop_words::{get, LANGUAGE as StopLanguage};
use whatlang::{detect, Lang};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationConfig {
    pub stop_words: HashMap<String, HashSet<String>>, // Stores stopwords by language code
    pub sentence_delimiters: HashSet<char>,
    pub paragraph_delimiters: String,
    pub default_ngram_size: u32,
    pub default_min_length: u32,
    pub default_similarity_threshold: f64,
    pub supported_languages: HashSet<String>,
}

impl Default for DeduplicationConfig {
    fn default() -> Self {
        let mut stop_words = HashMap::new();
        
        // Supported languages mapped to ISO codes
        let supported_langs = [
            (StopLanguage::English, "en"),
            (StopLanguage::French, "fr"),
            (StopLanguage::Spanish, "es"),
            (StopLanguage::German, "de"),
            (StopLanguage::Italian, "it"),
            (StopLanguage::Portuguese, "pt"),
            (StopLanguage::Russian, "ru"),
            (StopLanguage::Japanese, "ja"),
            (StopLanguage::Chinese, "zh"),
            (StopLanguage::Arabic, "ar"),
            (StopLanguage::Hindi, "hi"),
            (StopLanguage::Korean, "ko"),
            (StopLanguage::Dutch, "nl"),
            (StopLanguage::Swedish, "sv"),
            (StopLanguage::Turkish, "tr"),
        ];

        // Load stop words for supported languages
        for (lang, code) in &supported_langs {
            let words_set: HashSet<String> = get(lang.clone()).into_iter().collect();
            stop_words.insert(code.to_string(), words_set);
        }

        Self {
            stop_words,
            sentence_delimiters: vec!['.', '!', '?', '。', '！', '？', '।', '۔']
                .into_iter()
                .collect(),
            paragraph_delimiters: "\n\n".to_string(),
            default_ngram_size: 3,
            default_min_length: 10,
            default_similarity_threshold: 0.8,
            supported_languages: supported_langs.iter().map(|(_, code)| code.to_string()).collect(),
        }
    }
}

// ---------------------------------------------------------------------
// Dynamic Config
// ---------------------------------------------------------------------

/// **Global configuration that can be customized at runtime**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicConfig {
    pub base: DeduplicationConfig,
    pub user_stop_words: HashMap<String, HashSet<String>>, // Custom user-defined stop words
    pub user_sentence_delimiters: HashSet<char>,           // Additional user-defined sentence delimiters
    pub user_paragraph_delimiters: Option<String>,         // Custom paragraph delimiter
}

impl Default for DynamicConfig {
    fn default() -> Self {
        Self {
            base: DeduplicationConfig::default(),
            user_stop_words: HashMap::new(),
            user_sentence_delimiters: HashSet::new(),
            user_paragraph_delimiters: None,
        }
    }
}

impl DynamicConfig {
    /// **Merges system-defined and user-defined stop words.**
    pub fn merge_stop_words(&self, lang_code: &str) -> HashSet<String> {
        let base_words = self.base.stop_words.get(lang_code).cloned().unwrap_or_default();
        let user_words = self.user_stop_words.get(lang_code).cloned().unwrap_or_default();
        base_words.union(&user_words).cloned().collect()
    }

    /// **Returns merged stop words for detected language.**
    pub fn get_stop_words_for_text(&self, text: &str) -> HashSet<String> {
        // Detect language of the text
        let lang_code = detect(text)
            .map(|info| match info.lang() {
                Lang::Eng => "en",
                Lang::Fra => "fr",
                Lang::Spa => "es",
                Lang::Deu => "de",
                Lang::Ita => "it",
                Lang::Por => "pt",
                Lang::Rus => "ru",
                Lang::Jpn => "ja",
                Lang::Cmn => "zh",
                Lang::Ara => "ar",
                Lang::Hin => "hi",
                Lang::Kor => "ko",
                Lang::Nld => "nl",
                Lang::Swe => "sv",
                Lang::Tur => "tr",
                _ => "en", // Default to English if unsupported
            })
            .unwrap_or("en");

        self.merge_stop_words(lang_code)
    }

    /// **Returns merged sentence delimiters (base + user-defined).**
    pub fn merge_sentence_delimiters(&self) -> HashSet<char> {
        self.base
            .sentence_delimiters
            .union(&self.user_sentence_delimiters)
            .cloned()
            .collect()
    }

    /// **Returns the paragraph delimiter (user-defined or default).**
    pub fn get_paragraph_delimiters(&self) -> &str {
        self.user_paragraph_delimiters
            .as_deref()
            .unwrap_or(&self.base.paragraph_delimiters)
    }
}
