//! Pronunciation synthesis functionality.

use crate::error::Result;
use crate::models::Character;

/// Represents a phoneme with its pronunciation characteristics.
#[derive(Debug, Clone)]
pub struct Phoneme {
    /// IPA symbol
    pub symbol: String,
    
    /// Type of phoneme (vowel, consonant, etc)
    pub phoneme_type: PhonemeType,
    
    /// Descriptive text for the phoneme
    pub description: String,
}

/// Type of phoneme.
#[derive(Debug, Clone, PartialEq)]
pub enum PhonemeType {
    Vowel,
    Consonant,
    Fricative,
    Plosive,
    Nasal,
    Approximant,
    Diphthong,
    Other,
}

/// Text-to-Speech representation of a character's pronunciation.
#[derive(Debug, Clone)]
pub struct PronunciationGuide {
    pub symbol: String,
    pub ipa: String,
    pub description: Option<String>,
}

/// Parse IPA notation and extract phonetic information.
pub fn parse_ipa(ipa: &str) -> Result<Vec<Phoneme>> {
    let mut phonemes = Vec::new();
    
    // Remove slashes and extra whitespace
    let cleaned = ipa.trim_matches(|c| c == '/' || c == '[' || c == ']').trim();
    
    // Split by phoneme markers
    for phoneme_str in cleaned.split_whitespace() {
        if let Some(phoneme) = parse_single_phoneme(phoneme_str) {
            phonemes.push(phoneme);
        }
    }
    
    Ok(phonemes)
}

/// Parse a single phoneme from IPA notation.
fn parse_single_phoneme(ipa: &str) -> Option<Phoneme> {
    // Map common IPA symbols to phoneme types and descriptions
    let (phoneme_type, description) = match ipa {
        // Vowels
        "/ə/" | "ə" => (PhonemeType::Vowel, "Schwa (neutral vowel)"),
        "/ɑ/" | "ɑ" => (PhonemeType::Vowel, "Open back unrounded vowel"),
        "/æ/" | "æ" => (PhonemeType::Vowel, "Near-front unrounded vowel"),
        "/ɛ/" | "ɛ" => (PhonemeType::Vowel, "Open-mid front unrounded vowel"),
        "/ɪ/" | "ɪ" => (PhonemeType::Vowel, "Near-close front unrounded vowel"),
        "/oʊ/" | "oʊ" => (PhonemeType::Diphthong, "Close-mid back rounded vowel"),
        "/ʊ/" | "ʊ" => (PhonemeType::Vowel, "Near-close back rounded vowel"),
        "/u/" | "u" => (PhonemeType::Vowel, "Close back rounded vowel"),
        "/i/" | "i" => (PhonemeType::Vowel, "Close front unrounded vowel"),
        "/aɪ/" | "aɪ" => (PhonemeType::Diphthong, "Diphthong: a + i"),
        "/ɔɪ/" | "ɔɪ" => (PhonemeType::Diphthong, "Diphthong: o + i"),
        
        // Consonants - Plosives
        "/p/" | "p" => (PhonemeType::Plosive, "Voiceless bilabial plosive"),
        "/b/" | "b" => (PhonemeType::Plosive, "Voiced bilabial plosive"),
        "/t/" | "t" => (PhonemeType::Plosive, "Voiceless alveolar plosive"),
        "/d/" | "d" => (PhonemeType::Plosive, "Voiced alveolar plosive"),
        "/k/" | "k" => (PhonemeType::Plosive, "Voiceless velar plosive"),
        "/g/" | "g" => (PhonemeType::Plosive, "Voiced velar plosive"),
        
        // Consonants - Fricatives
        "/f/" | "f" => (PhonemeType::Fricative, "Voiceless labiodental fricative"),
        "/v/" | "v" => (PhonemeType::Fricative, "Voiced labiodental fricative"),
        "/θ/" | "θ" => (PhonemeType::Fricative, "Voiceless dental fricative"),
        "/ð/" | "ð" => (PhonemeType::Fricative, "Voiced dental fricative"),
        "/s/" | "s" => (PhonemeType::Fricative, "Voiceless alveolar fricative"),
        "/z/" | "z" => (PhonemeType::Fricative, "Voiced alveolar fricative"),
        "/ʃ/" | "ʃ" => (PhonemeType::Fricative, "Voiceless postalveolar fricative"),
        "/ʒ/" | "ʒ" => (PhonemeType::Fricative, "Voiced postalveolar fricative"),
        "/h/" | "h" => (PhonemeType::Fricative, "Voiceless glottal fricative"),
        
        // Consonants - Nasals
        "/m/" | "m" => (PhonemeType::Nasal, "Bilabial nasal"),
        "/n/" | "n" => (PhonemeType::Nasal, "Alveolar nasal"),
        "/ŋ/" | "ŋ" => (PhonemeType::Nasal, "Velar nasal"),
        
        // Consonants - Approximants/Semivowels
        "/w/" | "w" => (PhonemeType::Approximant, "Labiovelar approximant"),
        "/j/" | "j" => (PhonemeType::Approximant, "Palatal approximant"),
        "/l/" | "l" => (PhonemeType::Approximant, "Alveolar lateral approximant"),
        "/r/" | "r" => (PhonemeType::Approximant, "Alveolar approximant"),
        
        // Default for unknown
        _ => (PhonemeType::Other, "Unknown phoneme"),
    };
    
    Some(Phoneme {
        symbol: ipa.to_string(),
        phoneme_type,
        description: description.to_string(),
    })
}

/// Generate a pronunciation guide for a character.
pub fn generate_guide(character: &Character) -> PronunciationGuide {
    PronunciationGuide {
        symbol: character.symbol.clone(),
        ipa: character.pronunciation.clone(),
        description: character.description.clone(),
    }
}

/// Get verbal description of pronunciation (for text-to-speech simulation).
pub fn describe_pronunciation(ipa: &str) -> String {
    let phonemes = parse_ipa(ipa).unwrap_or_default();
    
    if phonemes.is_empty() {
        return format!("Pronunciation: {}", ipa);
    }
    
    let descriptions: Vec<String> = phonemes
        .iter()
        .map(|p| p.description.clone())
        .collect();
    
    format!("Pronounced as: {}", descriptions.join(" + "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ipa_vowel() {
        let phonemes = parse_ipa("/ə/").unwrap();
        assert_eq!(phonemes.len(), 1);
        assert_eq!(phonemes[0].phoneme_type, PhonemeType::Vowel);
    }

    #[test]
    fn test_parse_ipa_consonant() {
        let phonemes = parse_ipa("/k/").unwrap();
        assert_eq!(phonemes.len(), 1);
        assert_eq!(phonemes[0].phoneme_type, PhonemeType::Plosive);
    }

    #[test]
    fn test_parse_single_phoneme() {
        let phoneme = parse_single_phoneme("/p/").unwrap();
        assert_eq!(phoneme.phoneme_type, PhonemeType::Plosive);
        assert!(phoneme.description.contains("plosive"));
    }

    #[test]
    fn test_generate_guide() {
        let char = Character {
            symbol: "a".to_string(),
            pronunciation: "/ə/".to_string(),
            description: Some("vowel".to_string()),
        };
        
        let guide = generate_guide(&char);
        assert_eq!(guide.symbol, "a");
        assert_eq!(guide.ipa, "/ə/");
    }

    #[test]
    fn test_describe_pronunciation() {
        let desc = describe_pronunciation("/p/");
        assert!(desc.contains("Pronounced"));
    }
}
