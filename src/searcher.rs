//! Search functionality for dictionary lookups.

use crate::error::Result;
use crate::models::{DictionaryEntry, Language};
use regex::Regex;

/// Search options for flexible dictionary searching.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Case-insensitive search
    pub case_insensitive: bool,
    
    /// Use regular expressions
    pub use_regex: bool,
    
    /// Maximum number of results
    pub limit: Option<usize>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            case_insensitive: true,
            use_regex: false,
            limit: Some(50),
        }
    }
}

/// Perform a search operation on the language dictionary.
/// 
/// # Arguments
/// 
/// * `language` - The language to search in
/// * `term` - Search term
/// * `options` - Search options
/// 
/// # Returns
/// 
/// A `Result` containing search results
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tlict::searcher::{search, SearchOptions};
/// use tlict::language::Language;
/// 
/// // Search with default options
/// let results = search(&language, "example", &SearchOptions::default())?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn search(
    language: &Language,
    term: &str,
    options: &SearchOptions,
) -> Result<Vec<DictionaryEntry>> {
    if term.is_empty() {
        return Ok(vec![]);
    }

    let results = if options.use_regex {
        search_regex(language, term)?
    } else {
        search_text(language, term, options.case_insensitive)
    };

    let limited_results = if let Some(limit) = options.limit {
        results.into_iter().take(limit).collect()
    } else {
        results
    };

    Ok(limited_results)
}

/// Search using text matching.
fn search_text(language: &Language, term: &str, case_insensitive: bool) -> Vec<DictionaryEntry> {
    language
        .dictionary
        .iter()
        .filter(|(word, _)| {
            if case_insensitive {
                word.to_lowercase().contains(&term.to_lowercase())
            } else {
                word.contains(term)
            }
        })
        .map(|(word, definition)| DictionaryEntry {
            word: word.clone(),
            definition: definition.clone(),
        })
        .collect()
}

/// Search using regular expressions.
fn search_regex(language: &Language, pattern: &str) -> Result<Vec<DictionaryEntry>> {
    let regex = Regex::new(pattern)
        .map_err(|e| crate::error::TlictError::SearchError(e.to_string()))?;

    let results = language
        .dictionary
        .iter()
        .filter(|(word, _)| regex.is_match(word))
        .map(|(word, definition)| DictionaryEntry {
            word: word.clone(),
            definition: definition.clone(),
        })
        .collect();

    Ok(results)
}

/// Search for exact word match.
pub fn search_exact(language: &Language, word: &str) -> Option<DictionaryEntry> {
    language.lookup(word).map(|definition| DictionaryEntry {
        word: word.to_string(),
        definition,
    })
}

/// Search for words starting with a prefix.
pub fn search_prefix(language: &Language, prefix: &str) -> Vec<DictionaryEntry> {
    language
        .dictionary
        .iter()
        .filter(|(word, _)| word.starts_with(prefix))
        .map(|(word, definition)| DictionaryEntry {
            word: word.clone(),
            definition: definition.clone(),
        })
        .collect()
}

/// Search for words containing a substring.
pub fn search_contains(language: &Language, substring: &str) -> Vec<DictionaryEntry> {
    language
        .dictionary
        .iter()
        .filter(|(word, _)| word.contains(substring))
        .map(|(word, definition)| DictionaryEntry {
            word: word.clone(),
            definition: definition.clone(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::models::LanguageConfig;

    fn create_test_language() -> Language {
        let mut dict = HashMap::new();
        dict.insert("hello".to_string(), "greeting".to_string());
        dict.insert("help".to_string(), "assistance".to_string());
        dict.insert("world".to_string(), "planet".to_string());
        dict.insert("HELLO".to_string(), "greeting in uppercase".to_string());

        Language::new(
            "test",
            LanguageConfig {
                language_section: crate::models::LanguageSection {
                    name: "test".to_string(),
                },
                font: None,
                dict: None,
                helpers: None,
            },
            dict,
            vec![],
            None,
        )
    }

    #[test]
    fn test_search_text_case_insensitive() {
        let language = create_test_language();
        let results = search_text(&language, "hello", true);
        assert_eq!(results.len(), 2); // "hello" and "HELLO"
    }

    #[test]
    fn test_search_text_case_sensitive() {
        let language = create_test_language();
        let results = search_text(&language, "hello", false);
        assert_eq!(results.len(), 1); // only "hello"
    }

    #[test]
    fn test_search_exact() {
        let language = create_test_language();
        let result = search_exact(&language, "hello");
        assert!(result.is_some());
        assert_eq!(result.unwrap().definition, "greeting");
    }

    #[test]
    fn test_search_prefix() {
        let language = create_test_language();
        let results = search_prefix(&language, "hel");
        assert_eq!(results.len(), 2); // "hello" and "help"
    }

    #[test]
    fn test_search_contains() {
        let language = create_test_language();
        let results = search_contains(&language, "ll");
        assert_eq!(results.len(), 1); // only "hello"
    }

    #[test]
    fn test_search_with_limit() {
        let language = create_test_language();
        let options = SearchOptions {
            case_insensitive: true,
            use_regex: false,
            limit: Some(1),
        };
        let results = search(&language, "h", &options).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_regex() {
        let language = create_test_language();
        let options = SearchOptions {
            case_insensitive: false,
            use_regex: true,
            limit: None,
        };
        let results = search(&language, "^hello$", &options).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search_empty_term() {
        let language = create_test_language();
        let results = search(&language, "", &SearchOptions::default()).unwrap();
        assert_eq!(results.len(), 0);
    }
}
