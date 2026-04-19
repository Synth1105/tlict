use std::error::Error;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tlict::searcher::{self, SearchOptions};
use tlict::builder;
use tlict::output;
use tlict::pronunciation;
use tlict::archive;

/// A functional language analysis and compilation tool
#[derive(Parser, Debug)]
#[command(version, about = "A language development and analysis tool", long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build a language from source directory to .lang file
    Build {
        /// Language source directory
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory (default: current directory)
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },

    /// Search for terms in language dictionary
    Search {
        /// Search term or pattern
        term: String,

        /// Language archive file (.lang)
        #[arg(short, long)]
        lang: PathBuf,

        /// Use regex pattern matching
        #[arg(short = 'r', long)]
        regex: bool,

        /// Case-sensitive search
        #[arg(short = 'c', long)]
        case_sensitive: bool,

        /// Maximum number of results
        #[arg(short = 'l', long)]
        limit: Option<usize>,
    },

    /// Show language information
    Info {
        /// Language archive file (.lang)
        #[arg(short, long)]
        lang: PathBuf,
    },

    /// Validate language directory structure
    Validate {
        /// Language archive file (.lang)
        #[arg(short, long)]
        lang: PathBuf,
    },

    /// List all characters in the language
    Characters {
        /// Language archive file (.lang)
        #[arg(short, long)]
        lang: PathBuf,

        /// Show details for each character
        #[arg(short, long)]
        detailed: bool,
    },

    /// Speak/pronounce a character using IPA notation
    Speak {
        /// Character symbol or IPA pattern
        symbol: String,

        /// Language archive file (.lang)
        #[arg(short, long)]
        lang: PathBuf,

        /// Show detailed phoneme analysis
        #[arg(short, long)]
        detailed: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Build { input, output } => {
            if !input.exists() {
                output::error(&format!("Input directory not found: {}", input.display()));
                std::process::exit(1);
            }

            output::header(&format!("Building language from: {}", input.display()));
            match builder::build_language(&input, &output) {
                Ok(path) => {
                    output::success(&format!("Successfully built: {}", path.display()));
                    output::highlight(&format!("File size: {} bytes", 
                        std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0)));
                }
                Err(e) => {
                    output::error(&format!("Build failed: {}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Search {
            term,
            lang,
            regex,
            case_sensitive,
            limit,
        } => {
            match archive::load_from_lang_file(&lang) {
                Ok(language) => {
                    output::header(&format!("Searching in '{}'", language.name()));
                    output::highlight(&format!("Query: \"{}\"", term));
                    
                    let options = SearchOptions {
                        case_insensitive: !case_sensitive,
                        use_regex: regex,
                        limit,
                    };

                    match searcher::search(&language, &term, &options) {
                        Ok(results) => {
                            if results.is_empty() {
                                output::warning(&format!("No results found for: {}", term));
                            } else {
                                output::section(&format!("Found {} results", results.len()));
                                println!();
                                for (idx, entry) in results.iter().enumerate() {
                                    output::list_item(idx + 1, &format!("{}", entry.word));
                                    println!("    {}", entry.definition);
                                }
                            }
                        }
                        Err(e) => {
                            output::error(&format!("Search error: {}", e));
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    output::error(&format!("Failed to load language: {}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Info { lang } => {
            match archive::load_from_lang_file(&lang) {
                Ok(language) => {
                    output::header("Language Information");
                    output::pair("Name", language.name());
                    output::pair("Dictionary entries", &language.dictionary_size().to_string());
                    output::pair("Characters", &language.character_count().to_string());
                    output::pair("Font available", if language.has_font() { "Yes" } else { "No" });
                }
                Err(e) => {
                    output::error(&format!("Failed to load language: {}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Validate { lang } => {
            match archive::load_from_lang_file(&lang) {
                Ok(_language) => {
                    output::header("Validation Report");
                    println!();
                    output::checkmark("Language archive is valid");
                }
                Err(e) => {
                    output::error(&format!("Validation failed: {}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Characters {
            lang,
            detailed,
        } => {
            match archive::load_from_lang_file(&lang) {
                Ok(language) => {
                    if language.character_count() == 0 {
                        output::warning("No characters defined for this language");
                        return Ok(());
                    }

                    output::header(&format!("Characters in '{}' ({} total)", 
                        language.name(), language.character_count()));
                    println!();
                    
                    for (idx, char) in language.characters.iter().enumerate() {
                        output::list_item(idx + 1, &char.symbol);
                        output::pronunciation(&char.symbol, &char.pronunciation);
                        if detailed {
                            if let Some(desc) = &char.description {
                                println!("    {}", desc);
                            }
                        }
                        if idx < language.character_count() - 1 {
                            println!();
                        }
                    }
                }
                Err(e) => {
                    output::error(&format!("Failed to load language: {}", e));
                    std::process::exit(1);
                }
            }
        }

        Commands::Speak {
            symbol,
            lang,
            detailed,
        } => {
            match archive::load_from_lang_file(&lang) {
                Ok(language) => {
                    output::header("Pronunciation Guide");
                    println!();
                    
                    // Try to find the character
                    let found_char = language.characters
                        .iter()
                        .find(|c| c.symbol == symbol);
                    
                    if let Some(character) = found_char {
                        output::list_item(1, &character.symbol);
                        output::pronunciation(&character.symbol, &character.pronunciation);
                        
                        if let Some(desc) = &character.description {
                            println!("    {}", desc);
                        }
                        
                        println!();
                        output::section("IPA Analysis:");
                        let description = pronunciation::describe_pronunciation(&character.pronunciation);
                        println!("  {}", description);
                        
                        if detailed {
                            println!();
                            output::section("Detailed Phoneme Information:");
                            if let Ok(phonemes) = pronunciation::parse_ipa(&character.pronunciation) {
                                for (idx, phoneme) in phonemes.iter().enumerate() {
                                    output::list_item(idx + 1, &format!("{} ({})", 
                                        phoneme.symbol, 
                                        format!("{:?}", phoneme.phoneme_type).to_lowercase()));
                                    println!("      {}", phoneme.description);
                                }
                            }
                        }
                    } else {
                        output::warning(&format!("Character '{}' not found in language '{}'", symbol, language.name()));
                        
                        // Try to find similar characters
                        let similar: Vec<_> = language.characters
                            .iter()
                            .filter(|c| c.symbol.contains(&symbol) || symbol.contains(&c.symbol))
                            .collect();
                        
                        if !similar.is_empty() {
                            output::section("Did you mean:");
                            for (idx, char) in similar.iter().enumerate() {
                                output::list_item(idx + 1, &char.symbol);
                                output::pronunciation(&char.symbol, &char.pronunciation);
                            }
                        }
                    }
                }
                Err(e) => {
                    output::error(&format!("Failed to load language: {}", e));
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
