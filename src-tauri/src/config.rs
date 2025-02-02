use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use stop_words::{get, LANGUAGE};
use whatlang::Lang;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeduplicationConfig {
    pub stop_words: HashMap<String, HashSet<String>>,
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
        
        // Initialize stop words for all supported languages
        let supported_langs = [
            (LANGUAGE::English, "en"),
            (LANGUAGE::French, "fr"),
            (LANGUAGE::Spanish, "es"),
            (LANGUAGE::German, "de"),
            (LANGUAGE::Italian, "it"),
            (LANGUAGE::Portuguese, "pt"),
            (LANGUAGE::Russian, "ru"),
            (LANGUAGE::Japanese, "ja"),
            (LANGUAGE::Chinese, "zh"),
            (LANGUAGE::Arabic, "ar"),
            (LANGUAGE::Hindi, "hi"),
            (LANGUAGE::Korean, "ko"),
            (LANGUAGE::Dutch, "nl"),
            (LANGUAGE::Swedish, "sv"),
            (LANGUAGE::Turkish, "tr"),
        ];

        for (lang, code) in supported_langs.iter() {
            if let Ok(words) = get(*lang) {
                stop_words.insert(code.to_string(), words.into_iter().collect());
            }
        }

        Self {
            stop_words,
            sentence_delimiters: vec!['.', '!', '?', '。', '！', '？', '।', '۔'].into_iter().collect(),
            paragraph_delimiters: "\n\n".to_string(),
            default_ngram_size: 3,
            default_min_length: 10,
            default_similarity_threshold: 0.8,
            supported_languages: supported_langs.iter().map(|(_, code)| code.to_string()).collect(),
        }
    }
}

// Global configuration that can be customized at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicConfig {
    pub base: DeduplicationConfig,
    pub user_stop_words: HashMap<String, HashSet<String>>,
    pub user_sentence_delimiters: HashSet<char>,
    pub user_paragraph_delimiters: Option<String>,
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
    pub fn merge_stop_words(&self, lang_code: &str) -> HashSet<String> {
        let base_words = self.base.stop_words.get(lang_code).cloned().unwrap_or_default();
        let user_words = self.user_stop_words.get(lang_code).cloned().unwrap_or_default();
        base_words.union(&user_words).cloned().collect()
    }

    pub fn get_stop_words_for_text(&self, text: &str) -> HashSet<String> {
        // Detect language of the text
        if let Some(info) = whatlang::detect(text) {
            let lang_code = match info.lang() {
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
                _ => "en", // fallback to English
            };
            self.merge_stop_words(lang_code)
        } else {
            // Fallback to English if language detection fails
            self.merge_stop_words("en")
        }
    }

    pub fn merge_sentence_delimiters(&self) -> HashSet<char> {
        self.base.sentence_delimiters.union(&self.user_sentence_delimiters).cloned().collect()
    }

    pub fn get_paragraph_delimiters(&self) -> &str {
        self.user_paragraph_delimiters.as_deref().unwrap_or(&self.base.paragraph_delimiters)
    }
}
