//! # tlict - Language Analysis and Compilation Tool
//!
//! A functional Rust crate for constructing, analyzing, and managing domain-specific languages.
//! 
//! ## Features
//! 
//! - Build languages from TOML configuration and compress them into `.lang` files
//! - Search dictionary entries for language definitions
//! - Manage character definitions with pronunciation markers
//! - Support for custom fonts (OTF, TTF)
//!
//! ## Example
//!
//! ```rust,no_run
//! use tlict::language;
//! use std::path::Path;
//!
//! let language = language::load_from_path(Path::new("test-lang"))?;
//! println!("Language: {}", language.name());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod models;
pub mod language;
pub mod builder;
pub mod searcher;
pub mod font;
pub mod character;
pub mod error;

pub use models::{Language, LanguageConfig};
pub use error::{TlictError, Result};
