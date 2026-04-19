use std::error::Error;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tlict::language;
use tlict::searcher::{self, SearchOptions};
use tlict::builder;

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

        /// Language directory
        #[arg(short, long, default_value = ".")]
        lang_dir: PathBuf,

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
        /// Language directory
        #[arg(short, long, default_value = ".")]
        lang_dir: PathBuf,
    },

    /// Validate language directory structure
    Validate {
        /// Language directory to validate
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
    },

    /// List all characters in the language
    Characters {
        /// Language directory
        #[arg(short, long, default_value = ".")]
        lang_dir: PathBuf,

        /// Show details for each character
        #[arg(short, long)]
        detailed: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::Build { input, output } => {
            if !input.exists() {
                eprintln!("Error: Input directory not found: {}", input.display());
                std::process::exit(1);
            }

            println!("Building language from: {}", input.display());
            match builder::build_language(&input, &output) {
                Ok(path) => {
                    println!("✓ Successfully built: {}", path.display());
                }
                Err(e) => {
                    eprintln!("✗ Build failed: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Search {
            term,
            lang_dir,
            regex,
            case_sensitive,
            limit,
        } => {
            match language::load_from_path(&lang_dir) {
                Ok(lang) => {
                    let options = SearchOptions {
                        case_insensitive: !case_sensitive,
                        use_regex: regex,
                        limit,
                    };

                    match searcher::search(&lang, &term, &options) {
                        Ok(results) => {
                            if results.is_empty() {
                                println!("No results found for: {}", term);
                            } else {
                                println!("Found {} results:\n", results.len());
                                for (idx, entry) in results.iter().enumerate() {
                                    println!("{}. {}", idx + 1, entry.word);
                                    println!("   Definition: {}", entry.definition);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Search error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load language: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Info { lang_dir } => {
            match language::load_from_path(&lang_dir) {
                Ok(lang) => {
                    println!("Language: {}", lang.name());
                    println!("Dictionary entries: {}", lang.dictionary_size());
                    println!("Characters: {}", lang.character_count());
                    println!("Font available: {}", if lang.has_font() { "Yes" } else { "No" });
                }
                Err(e) => {
                    eprintln!("Failed to load language: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Validate { path } => {
            match builder::validate_language_dir(&path) {
                Ok(report) => {
                    println!("Validation Report for: {}\n", path.display());
                    println!("lang.toml: {}", if report.has_lang_toml { "✓" } else { "✗" });
                    println!(
                        "Dictionary: {} ({} files)",
                        if report.has_dict { "✓" } else { "✗" },
                        report.dict_files
                    );
                    println!("Characters: {}", if report.has_chars { "✓" } else { "✗" });
                    println!(
                        "Font: {} ({} files)",
                        if report.has_font { "✓" } else { "✗" },
                        report.font_files
                    );
                    println!(
                        "\nValid: {}",
                        if report.is_valid { "Yes ✓" } else { "No ✗" }
                    );
                }
                Err(e) => {
                    eprintln!("Validation error: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Characters {
            lang_dir,
            detailed,
        } => {
            match language::load_from_path(&lang_dir) {
                Ok(lang) => {
                    if lang.character_count() == 0 {
                        println!("No characters defined for this language");
                        return Ok(());
                    }

                    println!("Characters in {}: ({} total)\n", lang.name(), lang.character_count());
                    for (idx, char) in lang.characters.iter().enumerate() {
                        println!("{}. {}", idx + 1, char.symbol);
                        println!("   Pronunciation: {}", char.pronunciation);
                        if detailed {
                            if let Some(desc) = &char.description {
                                println!("   Description: {}", desc);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load language: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
