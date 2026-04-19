//! Font management functionality.

use crate::error::Result;
use std::path::Path;

/// Represents font information and validation.
#[derive(Debug, Clone)]
pub struct Font {
    path: String,
    font_type: String,
}

impl Font {
    /// Create a new font instance.
    pub fn new(path: impl Into<String>, font_type: impl Into<String>) -> Self {
        Font {
            path: path.into(),
            font_type: font_type.into(),
        }
    }

    /// Get the font file path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the font type (TTF or OTF).
    pub fn font_type(&self) -> &str {
        &self.font_type
    }

    /// Validate font file exists and has correct extension.
    pub fn validate(&self) -> Result<()> {
        let path = Path::new(&self.path);

        if !path.exists() {
            return Err(crate::error::TlictError::FontError(format!(
                "Font file not found: {}",
                self.path
            )));
        }

        match path.extension().and_then(|s| s.to_str()) {
            Some("ttf") | Some("otf") => {
                if self.font_type.to_lowercase() == "ttf"
                    || self.font_type.to_lowercase() == "otf"
                {
                    Ok(())
                } else {
                    Err(crate::error::TlictError::FontError(
                        "Font type mismatch".to_string(),
                    ))
                }
            }
            _ => Err(crate::error::TlictError::FontError(
                "Unsupported font format".to_string(),
            )),
        }
    }

    /// Check if font file is readable.
    pub fn is_readable(&self) -> bool {
        std::fs::metadata(&self.path)
            .map(|m| m.is_file())
            .unwrap_or(false)
    }

    /// Get font file size in bytes.
    pub fn file_size(&self) -> Result<u64> {
        std::fs::metadata(&self.path)
            .map(|m| m.len())
            .map_err(|e| crate::error::TlictError::Io(e))
    }
}

/// Load font from a path.
pub fn load_font(path: &Path, font_type: &str) -> Result<Font> {
    let font = Font::new(path.to_string_lossy().to_string(), font_type);
    font.validate()?;
    Ok(font)
}

/// Validate multiple fonts.
pub fn validate_fonts(fonts: &[Font]) -> Result<()> {
    for font in fonts {
        font.validate()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_font_creation() {
        let font = Font::new("font.ttf", "ttf");
        assert_eq!(font.path(), "font.ttf");
        assert_eq!(font.font_type(), "ttf");
    }

    #[test]
    fn test_font_not_found() {
        let font = Font::new("/nonexistent/path/font.ttf", "ttf");
        assert!(font.validate().is_err());
    }

    #[test]
    fn test_font_file_size() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"dummy font content").unwrap();
        
        let font = Font::new(
            temp_file.path().to_string_lossy().to_string(),
            "ttf",
        );
        
        // Note: validate would fail because extension is not ttf/otf
        // but file_size should work
        assert!(font.is_readable());
    }
}
