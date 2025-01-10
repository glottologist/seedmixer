use std::fs::File;

use crate::{errors::StorageError, seed::Seed, words::WordList};

/// A trait for types that can store a `Seed` somewhere.
pub trait SeedStorer {
    /// Stores the given `Seed`. Returns an error if storage fails.
    fn store(&self, seed: Seed) -> Result<(), StorageError>;
}

/// Stores seeds to a JSON file.
/// - If `override_file_name` is Some("name"), it writes to "name_seed.json".
/// - Otherwise, it writes to "seed.json".
pub struct FileSeedStorer {
    wl: WordList,
    override_file_name: Option<String>,
}

impl FileSeedStorer {
    /// Creates a new `FileSeedStorer` with a provided `WordList` and optional override file name.
    pub fn new(wl: WordList, override_file_name: Option<String>) -> Self {
        Self {
            wl,
            override_file_name,
        }
    }
}

impl SeedStorer for FileSeedStorer {
    fn store(&self, seed: Seed) -> Result<(), StorageError> {
        // Determine the file name
        let file_name = match &self.override_file_name {
            Some(n) => format!("{}_seed.json", n),
            None => "seed.json".to_string(),
        };

        // Open or create the file, returning an error if it fails
        let file = File::create(file_name).map_err(StorageError::UnableToReadWriteFile)?;

        // Convert each index in the seed to a word using the `WordList::find` method
        let seed_words: Vec<String> = seed.indices.iter().map(|i| self.wl.find(*i)).collect();

        // Write the list of words to the file as JSON
        let _ = serde_json::to_writer(file, &seed_words);
        Ok(())
    }
}

/// Sends seed to the terminal (stdout).
pub struct TerminalSeedStorer {
    wl: WordList,
}

impl TerminalSeedStorer {
    /// Creates a new `TerminalSeedStorer` with a given `WordList`.
    pub fn new(wl: WordList) -> Self {
        Self { wl }
    }
}

impl SeedStorer for TerminalSeedStorer {
    fn store(&self, seed: Seed) -> Result<(), StorageError> {
        // Convert seed indices to words
        let seed_words: Vec<String> = seed
            .indices
            .clone()
            .iter()
            .map(|i| self.wl.find(*i))
            .collect();

        println!("Seed is:");
        // Print each word on its own line
        for seed_word in seed_words {
            println!("{}", seed_word);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{seed::Seed, words::WordList};
    use std::fs;

    #[test]
    fn test_file_seed_storer_default_filename() {
        // Create a test Seed
        let indices = vec![
            1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16, 9u16, 10u16, 11u16, 12u16,
        ];

        let seed = Seed::new(indices).expect("This should be a valid seed");

        let storer = FileSeedStorer::new(WordList::default(), None);

        // Execute
        let result = storer.store(seed);
        assert!(result.is_ok(), "{}", format!("{:?}", result));

        // Check the output file "seed.json" in the current directory
        let contents = fs::read_to_string("seed.json").unwrap();
        assert!(contents.contains("ability"));
        assert!(contents.contains("able"));
        let _ = fs::remove_file("seed.json").expect("We should be able to delete the file");
    }

    #[test]
    fn test_file_seed_storer_override_filename() {
        // Create a test Seed
        let indices = vec![
            1u16, 2u16, 3u16, 4u16, 5u16, 6u16, 7u16, 8u16, 9u16, 10u16, 11u16, 12u16,
        ];

        let seed = Seed::new(indices).expect("This should be a valid seed");

        let storer = FileSeedStorer::new(WordList::default(), Some("custom".to_string()));

        // Execute
        let result = storer.store(seed);

        assert!(result.is_ok(), "{}", format!("{:?}", result));

        // Check the output file "seed.json" in the current directory
        let contents = fs::read_to_string("custom_seed.json").unwrap();
        assert!(contents.contains("ability"));
        assert!(contents.contains("able"));
        let _ = fs::remove_file("custom_seed.json").expect("We should be able to delete the file");
    }
}
