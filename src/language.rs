//! Language loading and parsing functionality.

use crate::error::Result;
use crate::models::{Character, FontInfo, Language, LanguageConfig};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Load language configuration from TOML file.
/// 
/// # Arguments
/// 
/// * `path` - Path to the language directory containing `lang.toml`
/// 
/// # Returns
/// 
/// A `Result` containing the loaded `Language`
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tlict::language::load_from_path;
/// use std::path::Path;
/// 
/// let language = load_from_path(Path::new("test-lang"))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load_from_path(path: &Path) -> Result<Language> {
    let lang_toml_path = path.join("lang.toml");
    
    if !lang_toml_path.exists() {
        return Err(crate::error::TlictError::LanguageNotFound(path.to_path_buf()));
    }

    let config_content = fs::read_to_string(&lang_toml_path)?;
    let config: LanguageConfig = toml::from_str(&config_content)?;
    
    let language_name = config.language_section.name.clone();
    
    // Load dictionary
    let dictionary = load_dictionary(path, &config)?;
    
    // Load characters
    let characters = load_characters(path, &config)?;
    
    // Load font information
    let font = load_font_info(path, &config)?;
    
    Ok(Language::new(
        language_name,
        config,
        dictionary,
        characters,
        font,
    ))
}

/// Load dictionary entries from files in the dictionary directory.
fn load_dictionary(base_path: &Path, config: &LanguageConfig) -> Result<HashMap<String, String>> {
    let mut dictionary = HashMap::new();

    if let Some(dict_section) = &config.dict {
        let dict_path = base_path.join(&dict_section.path);
        
        if !dict_path.exists() {
            return Err(crate::error::TlictError::DictionaryNotFound(dict_path));
        }

        // Load all JSON files from dictionary directory
        for entry in WalkDir::new(&dict_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "json").unwrap_or(false))
        {
            let content = fs::read_to_string(entry.path())?;
            let entries: HashMap<String, String> = serde_json::from_str(&content)?;
            dictionary.extend(entries);
        }
    }

    Ok(dictionary)
}

/// Load character definitions from characters directory.
fn load_characters(base_path: &Path, _config: &LanguageConfig) -> Result<Vec<Character>> {
    let mut characters = Vec::new();
    let chars_path = base_path.join("chars");

    if chars_path.exists() {
        let content = fs::read_to_string(&chars_path)?;
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            if let Some(character) = parse_character_line(line) {
                characters.push(character);
            }
        }
    }

    Ok(characters)
}

/// Parse a single character definition line.
/// 
/// Expected format: "symbol\tpronunciation[,description]"
fn parse_character_line(line: &str) -> Option<Character> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let parts: Vec<&str> = trimmed.split('\t').collect();
    if parts.len() < 2 {
        return None;
    }

    let symbol = parts[0].to_string();
    let pronunciation_and_desc = parts[1];
    
    let (pronunciation, description) = if let Some(comma_pos) = pronunciation_and_desc.find(',') {
        let (pron, desc) = pronunciation_and_desc.split_at(comma_pos);
        (pron.to_string(), Some(desc[1..].to_string()))
    } else {
        (pronunciation_and_desc.to_string(), None)
    };

    Some(Character {
        symbol,
        pronunciation,
        description,
    })
}

/// Load font information if available.
fn load_font_info(base_path: &Path, config: &LanguageConfig) -> Result<Option<FontInfo>> {
    if let Some(font_section) = &config.font {
        let font_path = base_path.join(&font_section.path);
        
        if !font_path.exists() {
            return Err(crate::error::TlictError::FontError(
                format!("Font file not found at: {}", font_path.display())
            ));
        }

        Ok(Some(FontInfo {
            path: font_path,
            font_type: font_section.font_type.clone(),
        }))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_language_dir() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        // Create lang.toml
        fs::write(
            base.join("lang.toml"),
            r#"[language]
name = "test-lang"

[dict]
path = "./dict"
"#,
        )
        .unwrap();

        // Create dict directory
        fs::create_dir(base.join("dict")).unwrap();

        // Create a test dictionary file
        let dict_content = r#"{"hello": "greeting", "world": "planet"}"#;
        fs::write(base.join("dict/basic.json"), dict_content).unwrap();

        temp_dir
    }

    #[test]
    fn test_load_from_path() {
        let temp_dir = create_test_language_dir();
        let language = load_from_path(temp_dir.path()).unwrap();
        
        assert_eq!(language.name(), "test-lang");
        assert_eq!(language.dictionary_size(), 2);
    }

    #[test]
    fn test_parse_character_line() {
        let line = "ㄱ\t/kɪ/,Korean consonant Giyeok";
        let char = parse_character_line(line).unwrap();
        
        assert_eq!(char.symbol, "ㄱ");
        assert_eq!(char.pronunciation, "/kɪ/");
        assert_eq!(char.description, Some("Korean consonant Giyeok".to_string()));
    }

    #[test]
    fn test_parse_character_line_without_description() {
        let line = "ㄱ\t/kɪ/";
        let char = parse_character_line(line).unwrap();
        
        assert_eq!(char.symbol, "ㄱ");
        assert_eq!(char.pronunciation, "/kɪ/");
        assert_eq!(char.description, None);
    }

    #[test]
    fn test_parse_character_line_empty() {
        assert!(parse_character_line("").is_none());
        assert!(parse_character_line("# comment").is_none());
    }
}
