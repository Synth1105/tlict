//! Character definition and management functionality.

use crate::error::Result;
use crate::models::Character;
use std::collections::HashMap;

/// Character set manager for a language.
#[derive(Debug, Clone)]
pub struct CharacterSet {
    characters: HashMap<String, Character>,
}

impl CharacterSet {
    /// Create a new character set.
    pub fn new() -> Self {
        CharacterSet {
            characters: HashMap::new(),
        }
    }

    /// Add a character to the set.
    pub fn add(&mut self, character: Character) {
        self.characters.insert(character.symbol.clone(), character);
    }

    /// Add multiple characters.
    pub fn add_multiple(&mut self, characters: Vec<Character>) {
        for character in characters {
            self.add(character);
        }
    }

    /// Get a character by symbol.
    pub fn get(&self, symbol: &str) -> Option<&Character> {
        self.characters.get(symbol)
    }

    /// Check if character exists.
    pub fn contains(&self, symbol: &str) -> bool {
        self.characters.contains_key(symbol)
    }

    /// Get all characters.
    pub fn all(&self) -> Vec<&Character> {
        self.characters.values().collect()
    }

    /// Get number of characters.
    pub fn count(&self) -> usize {
        self.characters.len()
    }

    /// Find characters by pronunciation.
    pub fn find_by_pronunciation(&self, pronunciation: &str) -> Vec<&Character> {
        self.characters
            .values()
            .filter(|c| c.pronunciation.contains(pronunciation))
            .collect()
    }

    /// Find characters by description pattern.
    pub fn find_by_description(&self, pattern: &str) -> Vec<&Character> {
        self.characters
            .values()
            .filter(|c| {
                c.description
                    .as_ref()
                    .map(|d| d.contains(pattern))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Export as JSON representation.
    pub fn to_json(&self) -> Result<String> {
        let chars: Vec<_> = self.characters.values().collect();
        serde_json::to_string_pretty(&chars)
            .map_err(|e| crate::error::TlictError::JsonError(e.to_string()))
    }
}

impl Default for CharacterSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate character definition format.
pub fn validate_character(symbol: &str, pronunciation: &str) -> Result<()> {
    if symbol.is_empty() {
        return Err(crate::error::TlictError::CharacterError(
            "Character symbol cannot be empty".to_string(),
        ));
    }

    if pronunciation.is_empty() {
        return Err(crate::error::TlictError::CharacterError(
            "Pronunciation cannot be empty".to_string(),
        ));
    }

    Ok(())
}

/// Create a character from components.
pub fn create_character(
    symbol: impl Into<String>,
    pronunciation: impl Into<String>,
    description: Option<impl Into<String>>,
) -> Result<Character> {
    let symbol = symbol.into();
    let pronunciation = pronunciation.into();

    validate_character(&symbol, &pronunciation)?;

    Ok(Character {
        symbol,
        pronunciation,
        description: description.map(|d| d.into()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_set_creation() {
        let set = CharacterSet::new();
        assert_eq!(set.count(), 0);
    }

    #[test]
    fn test_add_character() {
        let mut set = CharacterSet::new();
        let char = Character {
            symbol: "ㄱ".to_string(),
            pronunciation: "/kɪ/".to_string(),
            description: Some("Giyeok".to_string()),
        };

        set.add(char.clone());
        assert_eq!(set.count(), 1);
        assert!(set.contains("ㄱ"));
        assert_eq!(set.get("ㄱ").unwrap().symbol, "ㄱ");
    }

    #[test]
    fn test_add_multiple_characters() {
        let mut set = CharacterSet::new();
        let chars = vec![
            Character {
                symbol: "ㄱ".to_string(),
                pronunciation: "/kɪ/".to_string(),
                description: None,
            },
            Character {
                symbol: "ㄴ".to_string(),
                pronunciation: "/nɪ/".to_string(),
                description: None,
            },
        ];

        set.add_multiple(chars);
        assert_eq!(set.count(), 2);
    }

    #[test]
    fn test_find_by_pronunciation() {
        let mut set = CharacterSet::new();
        set.add(Character {
            symbol: "ㄱ".to_string(),
            pronunciation: "/kɪ/".to_string(),
            description: None,
        });
        set.add(Character {
            symbol: "a".to_string(),
            pronunciation: "/kɪ/".to_string(),
            description: None,
        });

        let results = set.find_by_pronunciation("/kɪ/");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_find_by_description() {
        let mut set = CharacterSet::new();
        set.add(Character {
            symbol: "ㄱ".to_string(),
            pronunciation: "/kɪ/".to_string(),
            description: Some("Korean consonant".to_string()),
        });
        set.add(Character {
            symbol: "a".to_string(),
            pronunciation: "/a/".to_string(),
            description: Some("Vowel".to_string()),
        });

        let results = set.find_by_description("Korean");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_validate_character() {
        assert!(validate_character("a", "/a/").is_ok());
        assert!(validate_character("", "/a/").is_err());
        assert!(validate_character("a", "").is_err());
    }

    #[test]
    fn test_create_character() {
        let char = create_character("ㄱ", "/kɪ/", Some("Giyeok")).unwrap();
        assert_eq!(char.symbol, "ㄱ");
        assert_eq!(char.pronunciation, "/kɪ/");
        assert_eq!(char.description, Some("Giyeok".to_string()));
    }
}
