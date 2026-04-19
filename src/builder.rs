//! Language building and compression functionality.

use crate::error::Result;
use crate::models::Language;
use std::fs::{self, File};
use std::path::Path;
use walkdir::WalkDir;

/// Build a language from source directory and compress to .lang file.
/// 
/// # Arguments
/// 
/// * `source_path` - Path to the language source directory
/// * `output_dir` - Directory where the .lang file will be created
/// 
/// # Returns
/// 
/// A `Result` with the path to the created .lang file
/// 
/// # Example
/// 
/// ```rust,no_run
/// use tlict::builder::build_language;
/// use std::path::Path;
/// 
/// let output_path = build_language(Path::new("test-lang"), Path::new("."))?;
/// println!("Built language: {}", output_path.display());
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn build_language(source_path: &Path, output_dir: &Path) -> Result<std::path::PathBuf> {
    // Load the language
    let language = crate::language::load_from_path(source_path)?;
    
    // Create a temporary directory for building
    let temp_dir = std::env::temp_dir().join(format!("tlict_build_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)?;

    // Create language archive with all files
    let archive_dir = temp_dir.join("language_data");
    fs::create_dir_all(&archive_dir)?;
    
    // Copy all files from source to archive directory
    copy_all_files(source_path, &archive_dir)?;
    
    // Add metadata.json
    let metadata = serde_json::json!({
        "name": language.name(),
        "dictionary_entries": language.dictionary_size(),
        "characters": language.character_count(),
        "has_font": language.has_font(),
        "built_at": chrono::Local::now().to_rfc3339(),
    });
    fs::write(archive_dir.join("metadata.json"), serde_json::to_string_pretty(&metadata)?)?;

    // Create output filename
    let lang_filename = format!("{}.lang", language.name());
    let output_path = output_dir.join(&lang_filename);

    // Create 7z archive
    create_7z_archive(&archive_dir, &output_path)?;

    // Clean up temp directory
    fs::remove_dir_all(&temp_dir)?;

    Ok(output_path)
}

/// Copy all files from source to destination directory.
fn copy_all_files(source: &Path, dest: &Path) -> Result<()> {
    for entry in WalkDir::new(source)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let relative = path.strip_prefix(source)
            .map_err(|e| crate::error::TlictError::BuildError(e.to_string()))?;
        
        let dest_path = dest.join(relative);
        
        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, &dest_path)?;
        }
    }
    
    Ok(())
}

/// Create a 7z compressed archive.
fn create_7z_archive(source_dir: &Path, output_path: &Path) -> Result<()> {
    use tar::Builder;
    
    // For now, create a tar.gz file as a compatible archive format
    // In production, this would use proper 7z compression
    let tar_path = output_path.with_extension("tar.gz");
    let tar_file = File::create(&tar_path)?;
    let encoder = flate2::write::GzEncoder::new(tar_file, flate2::Compression::default());
    let mut builder = Builder::new(encoder);
    
    // Add all files to the archive
    for entry in WalkDir::new(source_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let relative = path.strip_prefix(source_dir)
            .map_err(|e| crate::error::TlictError::BuildError(e.to_string()))?;
        
        if path.is_file() {
            builder.append_path_with_name(path, relative)
                .map_err(|e| crate::error::TlictError::BuildError(e.to_string()))?;
        }
    }
    
    builder.finish()
        .map_err(|e| crate::error::TlictError::BuildError(e.to_string()))?;
    
    // Rename to .lang
    fs::rename(&tar_path, output_path)?;
    
    Ok(())
}


/// Get language metadata for building.
pub fn get_metadata(language: &Language) -> LanguageMetadata {
    LanguageMetadata {
        name: language.name().to_string(),
        dictionary_entries: language.dictionary_size(),
        characters: language.character_count(),
        has_font: language.has_font(),
    }
}

/// Metadata about a built language.
#[derive(Debug, Clone)]
pub struct LanguageMetadata {
    pub name: String,
    pub dictionary_entries: usize,
    pub characters: usize,
    pub has_font: bool,
}

/// Validate a language directory structure.
pub fn validate_language_dir(path: &Path) -> Result<ValidationReport> {
    let mut report = ValidationReport::default();

    // Check for lang.toml
    if path.join("lang.toml").exists() {
        report.has_lang_toml = true;
    }

    // Check for dict directory
    if path.join("dict").exists() {
        report.has_dict = true;
        report.dict_files = count_json_files(&path.join("dict"));
    }

    // Check for chars file
    if path.join("chars").exists() {
        report.has_chars = true;
    }

    // Check for font directory
    if path.join("font").exists() {
        report.has_font = true;
        report.font_files = count_font_files(&path.join("font"));
    }

    report.is_valid = report.has_lang_toml && report.has_dict;

    Ok(report)
}

/// Count JSON files in a directory.
fn count_json_files(path: &Path) -> usize {
    if !path.exists() {
        return 0;
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|ext| ext == "json").unwrap_or(false))
        .count()
}

/// Count font files in a directory.
fn count_font_files(path: &Path) -> usize {
    if !path.exists() {
        return 0;
    }
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "ttf" || ext == "otf")
                .unwrap_or(false)
        })
        .count()
}

/// Report of language directory validation.
#[derive(Debug, Clone, Default)]
pub struct ValidationReport {
    pub has_lang_toml: bool,
    pub has_dict: bool,
    pub dict_files: usize,
    pub has_chars: bool,
    pub has_font: bool,
    pub font_files: usize,
    pub is_valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_language_structure() -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        let base = temp_dir.path();

        // Create lang.toml
        fs::write(
            base.join("lang.toml"),
            r#"[language]
name = "test-lang"
"#,
        )
        .unwrap();

        // Create dict directory with a JSON file
        fs::create_dir(base.join("dict")).unwrap();
        fs::write(base.join("dict/basic.json"), r#"{"hello": "greeting"}"#).unwrap();

        // Create chars file
        fs::write(base.join("chars"), "ㄱ\t/kɪ/").unwrap();

        temp_dir
    }

    #[test]
    fn test_validate_language_dir() {
        let temp_dir = create_test_language_structure();
        let report = validate_language_dir(temp_dir.path()).unwrap();

        assert!(report.has_lang_toml);
        assert!(report.has_dict);
        assert!(report.has_chars);
        assert_eq!(report.dict_files, 1);
        assert!(report.is_valid);
    }

    #[test]
    fn test_validate_invalid_dir() {
        let temp_dir = TempDir::new().unwrap();
        let report = validate_language_dir(temp_dir.path()).unwrap();

        assert!(!report.has_lang_toml);
        assert!(!report.is_valid);
    }

    #[test]
    fn test_get_metadata() {
        use std::collections::HashMap;
        
        let language = Language::new(
            "test",
            crate::models::LanguageConfig {
                language_section: crate::models::LanguageSection {
                    name: "test".to_string(),
                },
                font: None,
                dict: None,
                helpers: None,
            },
            {
                let mut dict = HashMap::new();
                dict.insert("hello".to_string(), "greeting".to_string());
                dict
            },
            vec![],
            None,
        );

        let metadata = get_metadata(&language);
        assert_eq!(metadata.name, "test");
        assert_eq!(metadata.dictionary_entries, 1);
    }
}
