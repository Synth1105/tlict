//! Error types for the tlict language processing system.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Result type alias for tlict operations.
pub type Result<T> = std::result::Result<T, TlictError>;

/// Comprehensive error type for tlict operations.
#[derive(Debug)]
pub enum TlictError {
    /// IO-related errors
    Io(io::Error),
    
    /// Configuration parsing errors
    ConfigParse(String),
    
    /// Language file not found
    LanguageNotFound(PathBuf),
    
    /// Dictionary file not found
    DictionaryNotFound(PathBuf),
    
    /// Font file not found or invalid
    FontError(String),
    
    /// Character definition error
    CharacterError(String),
    
    /// Search operation failed
    SearchError(String),
    
    /// Build/Compression error
    BuildError(String),
    
    /// JSON serialization error
    JsonError(String),
    
    /// TOML serialization error
    TomlError(String),
}

impl fmt::Display for TlictError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TlictError::Io(e) => write!(f, "IO error: {}", e),
            TlictError::ConfigParse(msg) => write!(f, "Configuration parse error: {}", msg),
            TlictError::LanguageNotFound(path) => {
                write!(f, "Language not found at: {}", path.display())
            }
            TlictError::DictionaryNotFound(path) => {
                write!(f, "Dictionary not found at: {}", path.display())
            }
            TlictError::FontError(msg) => write!(f, "Font error: {}", msg),
            TlictError::CharacterError(msg) => write!(f, "Character error: {}", msg),
            TlictError::SearchError(msg) => write!(f, "Search error: {}", msg),
            TlictError::BuildError(msg) => write!(f, "Build error: {}", msg),
            TlictError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            TlictError::TomlError(msg) => write!(f, "TOML error: {}", msg),
        }
    }
}

impl std::error::Error for TlictError {}

impl From<io::Error> for TlictError {
    fn from(err: io::Error) -> Self {
        TlictError::Io(err)
    }
}

impl From<serde_json::Error> for TlictError {
    fn from(err: serde_json::Error) -> Self {
        TlictError::JsonError(err.to_string())
    }
}

impl From<toml::de::Error> for TlictError {
    fn from(err: toml::de::Error) -> Self {
        TlictError::TomlError(err.to_string())
    }
}
