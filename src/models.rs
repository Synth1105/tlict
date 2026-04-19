//! Core data structures for language representation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a complete language with all its components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    /// Language name
    pub name: String,
    
    /// Language configuration
    pub config: LanguageConfig,
    
    /// Dictionary entries (word -> definition)
    pub dictionary: HashMap<String, String>,
    
    /// Character definitions
    pub characters: Vec<Character>,
    
    /// Font information
    pub font: Option<FontInfo>,
}

/// Language configuration loaded from TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageConfig {
    #[serde(rename = "language")]
    pub language_section: LanguageSection,
    
    #[serde(default)]
    pub font: Option<FontSection>,
    
    #[serde(default)]
    pub dict: Option<DictSection>,
    
    #[serde(default)]
    pub helpers: Option<HelpersSection>,
}

/// Language section in TOML.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSection {
    pub name: String,
}

/// Font configuration section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSection {
    pub path: String,
    #[serde(rename = "type")]
    pub font_type: String,
}

/// Dictionary configuration section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictSection {
    pub path: String,
}

/// Helpers configuration section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpersSection {
    #[serde(default)]
    pub search: Option<String>,
}

/// Font information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontInfo {
    pub path: PathBuf,
    pub font_type: String,
}

/// Represents a single character in the language.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Character {
    /// The character itself
    pub symbol: String,
    
    /// Pronunciation marker/IPA
    pub pronunciation: String,
    
    /// Optional description
    #[serde(default)]
    pub description: Option<String>,
}

/// Dictionary entry for search results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryEntry {
    pub word: String,
    pub definition: String,
}

impl Language {
    /// Create a new language.
    pub fn new(
        name: impl Into<String>,
        config: LanguageConfig,
        dictionary: HashMap<String, String>,
        characters: Vec<Character>,
        font: Option<FontInfo>,
    ) -> Self {
        Language {
            name: name.into(),
            config,
            dictionary,
            characters,
            font,
        }
    }

    /// Get the language name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the number of dictionary entries.
    pub fn dictionary_size(&self) -> usize {
        self.dictionary.len()
    }

    /// Get the number of characters.
    pub fn character_count(&self) -> usize {
        self.characters.len()
    }

    /// Check if the language has a font defined.
    pub fn has_font(&self) -> bool {
        self.font.is_some()
    }

    /// Get dictionary entry by word.
    pub fn lookup(&self, word: &str) -> Option<String> {
        self.dictionary.get(word).cloned()
    }

    /// Get all words in the dictionary that contain the search term.
    pub fn search_words(&self, term: &str) -> Vec<DictionaryEntry> {
        self.dictionary
            .iter()
            .filter(|(word, _)| word.to_lowercase().contains(&term.to_lowercase()))
            .map(|(word, definition)| DictionaryEntry {
                word: word.clone(),
                definition: definition.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_creation() {
        let language = Language::new(
            "test",
            LanguageConfig {
                language_section: LanguageSection {
                    name: "test".to_string(),
                },
                font: None,
                dict: None,
                helpers: None,
            },
            HashMap::new(),
            vec![],
            None,
        );

        assert_eq!(language.name(), "test");
        assert_eq!(language.dictionary_size(), 0);
        assert_eq!(language.character_count(), 0);
    }

    #[test]
    fn test_character_creation() {
        let char = Character {
            symbol: "ㄱ".to_string(),
            pronunciation: "/kɪ/".to_string(),
            description: Some("Korean consonant Giyeok".to_string()),
        };

        assert_eq!(char.symbol, "ㄱ");
        assert_eq!(char.pronunciation, "/kɪ/");
    }

    #[test]
    fn test_lookup() {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "greeting".to_string());

        let language = Language::new("test", LanguageConfig {
            language_section: LanguageSection {
                name: "test".to_string(),
            },
            font: None,
            dict: None,
            helpers: None,
        }, dict, vec![], None);

        assert_eq!(language.lookup("hello"), Some("greeting".to_string()));
        assert_eq!(language.lookup("world"), None);
    }

    #[test]
    fn test_search_words() {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "greeting".to_string());
        dict.insert("help".to_string(), "assistance".to_string());
        dict.insert("world".to_string(), "planet".to_string());

        let language = Language::new("test", LanguageConfig {
            language_section: LanguageSection {
                name: "test".to_string(),
            },
            font: None,
            dict: None,
            helpers: None,
        }, dict, vec![], None);

        let results = language.search_words("hel");
        assert_eq!(results.len(), 2);
    }
}
