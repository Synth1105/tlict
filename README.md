# tlict - Language Analysis and Compilation Tool

![Website](https://img.shields.io/website?url=https%3A%2F%2Ftlict-docs.vercel.app&up_message=https%3A%2F%2Ftlict-docs.vercel.app&style=for-the-badge&labelColor=%23393552&color=%23232136)
![Crates.io Downloads (recent)](https://img.shields.io/crates/dr/tlict?style=for-the-badge&labelColor=%23393552&color=%23232136)


A functional Rust crate for constructing, analyzing, and managing domain-specific languages (DSLs).

## Features

- **Build Languages**: Compile language definitions from source directories into compressed `.lang` files (tar.gz format)
- **Dictionary Search**: Powerful search capabilities with regex support and multiple search strategies
- **Character Management**: Define and manage language characters with IPA pronunciation markers
- **Pronunciation Synthesis**: Speak/pronounce characters with detailed IPA analysis
- **Font Support**: Handle custom fonts (OTF, TTF) for language rendering
- **Validation**: Comprehensive validation for language archive structures
- **Colorful Output**: Beautiful, colorful CLI output with formatted messages
- **Functional Design**: Pure functional Rust implementation with minimal side effects

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tlict = "0.1"
```

## Usage

### Command Line Interface

#### Build a language

```bash
tlict build --input test-lang --output .
```

This compiles the `test-lang` directory into a `test-lang.lang` file (tar.gz archive containing all files).

**Output:**
```
Building language from: test-lang
────────────────────────────────────
Successfully built: test-lang.lang
File size: 81912 bytes
```

#### Search in dictionary

```bash
tlict search "example" --lang test-lang.lang
```

Search options:
- `-r, --regex`: Use regular expression pattern matching
- `-c, --case-sensitive`: Perform case-sensitive search
- `-l, --limit <N>`: Limit results to N entries (default: 50)

**Output:**
```
Searching in 'test-lang'
────────────────────────
Query: "example"
Found 1 results

1. example
    A thing characteristic of its kind
```

#### Show language information

```bash
tlict info --lang test-lang.lang
```

**Output:**
```
Language Information
────────────────────
  Name test-lang
  Dictionary entries 15
  Characters 16
  Font available Yes
```

#### Validate language archive

```bash
tlict validate --lang test-lang.lang
```

**Output:**
```
Validation Report
─────────────────

[OK] Language archive is valid
```

#### List characters

```bash
tlict characters --lang test-lang.lang --detailed
```

**Output:**
```
Characters in 'test-lang' (16 total)
────────────────────────────────────

1. a
  a -> /ə/
    Latin vowel

2. e
  e -> /ɛ/
    Latin vowel
```

#### Speak/Pronounce a character

```bash
tlict speak "a" --lang test-lang.lang --detailed
```

**Output:**
```
Pronunciation Guide
───────────────────

1. a
  a -> /ə/
    Latin vowel

IPA Analysis:
  Pronounced as: Schwa (neutral vowel)

Detailed Phoneme Information:
1. ə (vowel)
      Schwa (neutral vowel)
```

```

#### Validate language structure

```bash
tlict validate --path test-lang
```

**Output:**
```
✓ Validation Report
─────────────────
Directory: test-lang

✓ lang.toml
✓ Dictionary (2 files)
✓ Characters
✓ Font (0 files)

✓ Structure is valid!
```

#### List characters

```bash
tlict characters --lang-dir test-lang --detailed
```

**Output:**
```
🔤 Characters in 'test-lang' (16 total)

1. a
  a → /ə/
  📝 Latin vowel

2. e
  e → /ɛ/
  📝 Latin vowel
```

### Library Usage

```rust
use tlict::archive;
use tlict::searcher::{self, SearchOptions};

// Load a language from .lang file
let lang = archive::load_from_lang_file(std::path::Path::new("test-lang.lang"))?;

// Search for terms
let options = SearchOptions::default();
let results = searcher::search(&lang, "hello", &options)?;

// Access language properties
println!("Language: {}", lang.name());
println!("Dictionary size: {}", lang.dictionary_size());
println!("Characters: {}", lang.character_count());
```

## Language Archive Structure

A `.lang` file is a tar.gz archive containing the following structure:

```
test-lang.lang (tar.gz format)
├── lang.toml           # Language configuration
├── chars              # Character definitions (tab-separated)
├── metadata.json      # Build metadata
├── dict/              # Dictionary files (JSON format)
│   ├── basic.json
│   ├── advanced.json
│   └── ...
└── font/              # Font files (OTF/TTF)
    ├── regular.ttf
    └── bold.ttf
```

```

### lang.toml Format

```toml
[language]
name = "test-lang"

[font]
path = "./font/font.ttf"
type = "ttf"

[dict]
path = "./dict/"

[helpers]
search = "./programs/search"
```

### Dictionary Format (JSON)

```json
{
  "word": "definition",
  "hello": "greeting expression",
  "world": "planet Earth"
}
```

### Character Definitions Format

Tab-separated values with optional description:

```
symbol<TAB>pronunciation[,description]
ㄱ	/kɪ/,Korean consonant Giyeok
ㄴ	/nɪ/,Korean consonant Nieun
a	/ɑ/,Latin vowel
```

## API Documentation

### Core Modules

#### `models`

Defines core data structures:
- `Language`: Complete language representation
- `LanguageConfig`: Configuration from TOML
- `Character`: Individual character definition
- `DictionaryEntry`: Dictionary entry with word and definition

#### `language`

Language loading and parsing:
- `load_from_path()`: Load language from directory

#### `searcher`

Dictionary search operations:
- `search()`: Perform search with options
- `search_exact()`: Find exact word match
- `search_prefix()`: Find words with prefix
- `search_contains()`: Find words containing substring

#### `builder`

Language building and validation:
- `build_language()`: Compile language to .lang file (tar.gz)
- `validate_language_dir()`: Validate directory structure
- `get_metadata()`: Extract language metadata

#### `character`

Character management:
- `CharacterSet`: Collection of characters
- `validate_character()`: Validate character definition
- `create_character()`: Create new character

#### `font`

Font management:
- `Font`: Font representation
- `load_font()`: Load and validate font file
- `validate_fonts()`: Validate multiple fonts

#### `pronunciation`

Pronunciation analysis:
- `parse_ipa()`: Parse IPA notation into phonemes
- `generate_guide()`: Create pronunciation guide
- `describe_pronunciation()`: Generate text description
- `Phoneme`: Detailed phoneme information

#### `archive` (NEW)

Language archive handling:
- `extract_lang_file()`: Extract .lang archive to temporary directory
- `load_from_lang_file()`: Load language directly from .lang file

#### `output`

Colorful CLI output utilities:
- `success()`: Green success messages
- `error()`: Red error messages with [ERROR] prefix
- `warning()`: Yellow warning messages with [WARNING] prefix
- `info()`: Blue info messages
- `header()`: Formatted section headers
- `pronunciation()`: Styled pronunciation display



## Examples

### Create a new language programmatically

```rust
use tlict::models::{Language, LanguageConfig, LanguageSection, Character};
use std::collections::HashMap;

let config = LanguageConfig {
    language_section: LanguageSection {
        name: "my-lang".to_string(),
    },
    font: None,
    dict: None,
    helpers: None,
};

let mut dict = HashMap::new();
dict.insert("hello".to_string(), "greeting".to_string());

let characters = vec![
    Character {
        symbol: "a".to_string(),
        pronunciation: "/ɑ/".to_string(),
        description: Some("vowel".to_string()),
    },
];

let language = Language::new("my-lang", config, dict, characters, None);

println!("Created language: {}", language.name());
println!("Words: {}", language.dictionary_size());
println!("Characters: {}", language.character_count());
```

### Search with different options

```rust
use tlict::searcher::{self, SearchOptions};

// Case-insensitive search
let options = SearchOptions {
    case_insensitive: true,
    use_regex: false,
    limit: Some(10),
};

let results = searcher::search(&lang, "hel", &options)?;

// Regex search
let regex_options = SearchOptions {
    case_insensitive: false,
    use_regex: true,
    limit: None,
};

let regex_results = searcher::search(&lang, "^[aeiou].*", &regex_options)?;
```

### Validate and get information

```rust
use tlict::builder;

// Validate directory structure
let report = builder::validate_language_dir(std::path::Path::new("test-lang"))?;

if report.is_valid {
    let metadata = builder::get_metadata(&lang);
    println!("Language: {}", metadata.name);
    println!("Dictionary: {} entries", metadata.dictionary_entries);
    println!("Characters: {}", metadata.characters);
}
```

## Performance Characteristics

- **Language Loading**: O(n) where n is the total size of dictionary and character files
- **Search**: 
  - Text search: O(m) where m is the number of dictionary entries
  - Regex search: O(m × p) where p is the regex pattern complexity
- **Character Lookup**: O(1) average case with HashMap

## Testing

Run all tests:

```bash
cargo test
```

Run specific test module:

```bash
cargo test language::tests
cargo test searcher::tests
```

Run with output:

```bash
cargo test -- --nocapture
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass (`cargo test`)
2. Code is formatted (`cargo fmt`)
3. No clippy warnings (`cargo clippy`)
4. Documentation is updated
5. New features include tests and docs

## Design Philosophy

tlict follows functional programming principles:

- **Immutability**: Data structures are immutable by default
- **Pure Functions**: Functions have minimal side effects
- **Composition**: Small functions combine to form larger operations
- **Error Handling**: Comprehensive `Result` types for error propagation
- **Type Safety**: Rust's type system prevents entire classes of bugs

## Roadmap

- [ ] Proper 7z compression support
- [ ] Advanced text analytics (n-grams, frequency analysis)
- [ ] Language statistics and reporting
- [ ] Font subset optimization
- [ ] Multi-language corpus support
- [ ] Performance benchmarking suite
- [ ] Web API support

## Support

For issues, questions, or suggestions, please open an issue on GitHub.
