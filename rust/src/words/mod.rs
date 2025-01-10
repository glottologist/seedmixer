use derive_more;
use std::collections::HashMap;

/// Constant for the size of the word list used for seed phrases, in BIP-39 implementations.
pub const WORDLISTSIZE: u16 = 2048;

/// Enum representing various languages supported by the word list.
#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub enum Language {
    ChineseSimplified,
    ChineseTraditional,
    Czech,
    English,
    French,
    Italian,
    Japanese,
    Korean,
    Portuguese,
    Spanish,
}

/// Structure to manage word lists for different languages, associating each word to a unique index.
#[derive(Debug, Clone)]
pub struct WordList {
    pub words: HashMap<u16, String>,
    pub inverted: HashMap<String, u16>,
    pub lang: Language,
}

impl Default for WordList {
    fn default() -> Self {
        Self::new("en")
    }
}

impl WordList {
    /// Static strings holding the content of word lists for each supported language.
    const CSE: &'static str = include_str!("../data/words/cse/words.txt");
    const ENG: &'static str = include_str!("../data/words/eng/words.txt");
    const FRA: &'static str = include_str!("../data/words/fra/words.txt");
    const ITA: &'static str = include_str!("../data/words/ita/words.txt");
    const JPN: &'static str = include_str!("../data/words/jpn/words.txt");
    const KOR: &'static str = include_str!("../data/words/kor/words.txt");
    const POR: &'static str = include_str!("../data/words/por/words.txt");
    const SPA: &'static str = include_str!("../data/words/spa/words.txt");
    const ZHOHANS: &'static str = include_str!("../data/words/zho-hans/words.txt");
    const ZHOHANT: &'static str = include_str!("../data/words/zho-hant/words.txt");

    /// Function to create a hashmap of words from a word list string based on the selected language.
    fn get_associated_word_list(language: &Language) -> HashMap<u16, String> {
        let wordlist = match language {
            Language::Czech => Self::CSE,
            Language::English => Self::ENG,
            Language::French => Self::FRA,
            Language::Italian => Self::ITA,
            Language::Japanese => Self::JPN,
            Language::Korean => Self::KOR,
            Language::Portuguese => Self::POR,
            Language::Spanish => Self::SPA,
            Language::ChineseSimplified => Self::ZHOHANS,
            Language::ChineseTraditional => Self::ZHOHANT,
        };

        // Parse the word list string into a vector, then into a HashMap associating each word with an index.
        let words: Vec<String> = wordlist.split_whitespace().map(str::to_string).collect();
        words
            .into_iter()
            .enumerate()
            .map(|(i, w)| (i as u16, w))
            .collect()
    }

    /// Method to determine the enum value for a language from a string identifier.
    fn get_language(language: &str) -> Language {
        match language {
            "zho-hans" => Language::ChineseSimplified,
            "zho-hant" => Language::ChineseTraditional,
            "cse" => Language::Czech,
            "eng" => Language::English,
            "fra" => Language::French,
            "ita" => Language::Italian,
            "jpn" => Language::Japanese,
            "kor" => Language::Korean,
            "por" => Language::Portuguese,
            "spa" => Language::Spanish,
            _ => Language::English, // Default to English if no match is found.
        }
    }

    /// Constructor for the WordList structure, initializes a WordList with words for a specific language.
    pub fn new(language: &str) -> Self {
        let lan = Self::get_language(language);
        let converted_words = Self::get_associated_word_list(&lan);
        let inverted: HashMap<String, u16> = converted_words
            .clone()
            .into_iter()
            .map(|(i, w)| (w, i))
            .collect();
        WordList {
            words: converted_words,
            inverted,
            lang: lan,
        }
    }

    /// Method to retrieve a word by its index from the word list.
    pub fn get_index(&self, word: &str) -> &u16 {
        //This is an intentional  panic. If any index can't be found, then everything should fail.
        self.inverted.get(word).unwrap()
    }

    /// Method to retrieve an index by its word from the word list.
    pub fn find(&self, index: u16) -> String {
        //This is an intentional  panic. If any word can't be found, then everything should fail.
        self.words.get(&index).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_to_english() {
        // If we pass an unknown language string, it should default to English.
        let wl = WordList::new("not-a-language");
        assert_eq!(wl.lang, Language::English, "Should default to English.");
    }

    #[test]
    fn test_english_wordlist_size() {
        // Ensure the English word list has 2048 words (BIP-39 standard).
        let wl = WordList::new("eng");
        assert_eq!(
            wl.words.len(),
            2048,
            "English word list should have 2048 words."
        );
    }

    #[test]
    fn test_forward_and_reverse_lookup() {
        let wl = WordList::new("eng");
        // Test the first word
        let first_word = wl.find(0);
        let index = wl.get_index(&first_word);
        assert_eq!(*index, 0, "Index of the first word should be 0.");

        // Test the last word
        let last_word = wl.find(2047);
        let index = wl.get_index(&last_word);
        assert_eq!(*index, 2047, "Index of the last word should be 2047.");
    }

    #[test]
    #[should_panic]
    fn test_panic_on_invalid_word() {
        let wl = WordList::new("eng");
        // This should panic because "nonexistentword" is not in the word list.
        let _ = wl.get_index("nonexistentword");
    }

    #[test]
    #[should_panic]
    fn test_panic_on_invalid_index() {
        let wl = WordList::new("eng");
        // This should panic because index 9999 doesn't exist in a 2048-sized list.
        let _ = wl.find(9999);
    }
}
