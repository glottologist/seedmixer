use crate::errors::SeedPhraseError;
use crate::words::{WordList, WORDLISTSIZE};
use std::io;
use std::{cmp::PartialEq, fmt};

/// Represents a seed, essentially a unique vector of indices and a length.
#[derive(Debug, PartialEq)]
pub struct Seed {
    /// The unique set of indices corresponding to words in the BIP-39 list.
    pub indices: Vec<u16>,
    /// The number of indices in the seed. Must be 12, 16, or 24 for valid seeds.
    pub length: u16,
}

impl Seed {
    /// Validates the seed’s indices to ensure they are within the valid bounds of the word list.
    fn validate_seed_indices(indices: Vec<u16>) -> Result<Self, SeedPhraseError> {
        // Indices should be between 1 and WORDLISTSIZE (2048) inclusive
        for index in indices.iter() {
            if index > &WORDLISTSIZE || index == &0 {
                return Err(SeedPhraseError::SeedPhraseIndexIsOutsideTheBoundsOfTheWordList);
            }
        }

        // If everything is valid, create the Seed.
        Ok(Seed {
            length: indices.len() as u16,
            indices,
        })
    }

    /// Public constructor that checks if the length of the seed is among the accepted sizes.
    pub fn new(seed_word_indices: Vec<u16>) -> Result<Self, SeedPhraseError> {
        let length = seed_word_indices.len();
        match &length {
            12 | 16 | 24 => Self::validate_seed_indices(seed_word_indices),
            _ => Err(SeedPhraseError::SeedPhraseLengthIsIncorrect(length)),
        }
    }
}

///Implementation to format the Seed data as a comma-separated list of indices.
impl fmt::Display for Seed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.indices
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

/// A structure to manage a seed input via the terminal.
#[derive(Debug)]
pub struct TerminalSeed {
    /// The user-input words from stdin.
    contents: Vec<String>,
    /// The associated word list (for mapping words to their indices).
    word_list: WordList,
}

impl TerminalSeed {
    /// Prompts the user for space-separated words, reads them from stdin,
    /// and stores them along with a reference to the provided `word_list`.
    pub fn new(word_list: WordList) -> Result<Self, SeedPhraseError> {
        println!("Please enter a list of words separated by spaces (please ensure words are correctly spelled):");

        // Read seed words from stdin
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(SeedPhraseError::UnableToDetermineTerminalInput)?;

        // Trim and split the input into words
        let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        Ok(TerminalSeed {
            contents: words,
            word_list,
        })
    }
}

/// Convert Terminal Seed to the more generic Seed
impl From<TerminalSeed> for Seed {
    /// For each word in contents, we look up its index in the `word_list`.
    fn from(value: TerminalSeed) -> Self {
        let indices: Vec<u16> = value
            .contents
            .clone()
            .into_iter()
            .map(|w| value.word_list.get_index(&w))
            .copied()
            .collect();

        let length = indices.len() as u16;

        Self { indices, length }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prop_assert;

    use {
        super::*,
        proptest::{
            prelude::{ProptestConfig, Strategy},
            proptest,
        },
    };

    // A strategy for valid seeds: only 12, 16, or 24 unique indices in [1..WORDLISTSIZE].
    fn valid_seed_strategy() -> impl Strategy<Value = Vec<u16>> {
        proptest::collection::vec(1u16..=WORDLISTSIZE, 1..=24)
            .prop_filter("Must have lengths 12,16 or 24", |s| {
                s.len() == 12 || s.len() == 16 || s.len() == 24
            })
    }

    // A strategy for invalid seeds: 1..24 unique indices, but NOT length 12,16, or 24.
    fn invalid_seed_strategy() -> impl Strategy<Value = Vec<u16>> {
        proptest::collection::vec(1u16..=WORDLISTSIZE, 1..24)
            .prop_filter("Must not have lengths 12 or 16", |s| {
                s.len() != 12 && s.len() != 16
            })
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]
        #[test]
        fn test_should_create_seed_prop(seed in valid_seed_strategy()) {
        let v:&Vec<u16>=&seed;
        let s = Seed::new(v.clone());
        prop_assert!(&s.is_ok(), "Seed should be constructable for a valid seed: {:?}", &seed);
        }

        #[test]
        fn test_new_seed_should_fail_for_invalid_seeds_prop(seed in invalid_seed_strategy()) {
        let s = Seed::new(seed.clone());
        prop_assert!(
            s.is_err(),
            "Seed construction should fail if the seed words are invalid: {:?}", &seed
        );
        }
    }
}
