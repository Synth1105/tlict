//! Language archive extraction functionality.

use crate::error::Result;
use std::fs::{self, File};
use std::path::Path;
use tar::Archive;

/// Extract a .lang file to a temporary directory.
///
/// # Arguments
///
/// * `lang_path` - Path to the .lang file
///
/// # Returns
///
/// A `Result` with the path to the extracted directory
pub fn extract_lang_file(lang_path: &Path) -> Result<std::path::PathBuf> {
    if !lang_path.exists() {
        return Err(crate::error::TlictError::LanguageNotFound(lang_path.to_path_buf()));
    }

    // Verify it's a valid .lang file
    if lang_path.extension().and_then(|s| s.to_str()) != Some("lang") {
        return Err(crate::error::TlictError::BuildError(
            "File must have .lang extension".to_string(),
        ));
    }

    // Create temporary extraction directory
    let temp_dir = std::env::temp_dir().join(format!("tlict_extract_{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)?;

    // Open and extract the tar.gz file
    let file = File::open(lang_path)?;
    let decoder = flate2::read::GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    
    archive.unpack(&temp_dir)
        .map_err(|e| crate::error::TlictError::BuildError(e.to_string()))?;

    Ok(temp_dir)
}

/// Load a language from a .lang file.
pub fn load_from_lang_file(lang_path: &Path) -> Result<crate::models::Language> {
    let extracted_dir = extract_lang_file(lang_path)?;
    crate::language::load_from_path(&extracted_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_nonexistent_file() {
        let result = extract_lang_file(Path::new("/nonexistent/file.lang"));
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_wrong_extension() {
        let temp_file = tempfile::NamedTempFile::new().unwrap();
        let result = extract_lang_file(temp_file.path());
        assert!(result.is_err());
    }
}
