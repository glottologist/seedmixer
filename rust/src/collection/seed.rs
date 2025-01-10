use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::{errors::CollectionError, seed::Seed, words::WordList};

/// A trait for collecting `Seed` data from different sources.
pub trait SeedCollector {
    /// Attempts to collect a `Seed` and returns an error if unsuccessful.
    fn collect(&self) -> Result<Seed, CollectionError>;
}

/// Collector that reads seed words from a file on disk.
pub struct FileSeedCollector {
    /// The file path that contains the seed words.
    path: PathBuf,
    /// A reference to a `WordList` that can map words to indices.
    wl: WordList,
}

impl FileSeedCollector {
    /// Constructs a new `FileSeedCollector` with a given `WordList` and file path.
    pub fn new(wl: WordList, path: PathBuf) -> Self {
        Self { path, wl }
    }
}

impl SeedCollector for FileSeedCollector {
    fn collect(&self) -> Result<Seed, CollectionError> {
        // Open the file for reading.
        let file = File::open(&self.path)
            .map_err(|_| CollectionError::UnableToReadSeedFromFile(self.path.clone()))?;

        // Wrap it in a buffered reader for efficient line reading.
        let reader = BufReader::new(file);

        // Collect all lines into a Vec<String>, returning an error if we fail to read lines.
        let contents = reader
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| CollectionError::UnableToReadSeedFromFile(self.path.clone()))?;

        // Convert each line/word into an index by querying the WordList.
        let indices: Vec<u16> = contents
            .into_iter()
            .map(|w| self.wl.get_index(&w))
            .copied()
            .collect();

        // The length of the seed is how many words (indices) were read.
        let length = indices.len() as u16;

        // Return a new Seed object.
        Ok(Seed { indices, length })
    }
}

/// Collector that reads seed words from the terminal (stdin).
pub struct TerminalSeedCollector {
    /// A reference to a `WordList` for mapping words to indices.
    wl: WordList,
}
impl TerminalSeedCollector {
    /// Constructs a new `TerminalSeedCollector` with a given `WordList`.
    pub fn new(wl: WordList) -> Self {
        Self { wl }
    }
}

impl SeedCollector for TerminalSeedCollector {
    fn collect(&self) -> Result<Seed, CollectionError> {
        println!("Please enter a list of words separated by spaces:");

        // Read from stdin; map any IO errors to our custom `CollectionError`.
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Split by whitespace, convert to indices, and store as raw_indices.
        let indices: Vec<u16> = input
            .split_whitespace()
            .map(|w| self.wl.get_index(w))
            .copied()
            .collect();

        let length = indices.len() as u16;

        // Return a Seed with the processed indices.
        Ok(Seed { indices, length })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_file_seed_collector_valid_file() {
        // Create a temporary directory
        let tmp_dir = TempDir::new().unwrap();
        let file_path = tmp_dir.path().join("seed.txt");

        // Write some words to the file
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "ability").unwrap();
            writeln!(file, "able").unwrap();
            writeln!(file, "about").unwrap();
            writeln!(file, "above").unwrap();
            writeln!(file, "absent").unwrap();
            writeln!(file, "absorb").unwrap();
            writeln!(file, "abstract").unwrap();
            writeln!(file, "absurd").unwrap();
            writeln!(file, "abuse").unwrap();
            writeln!(file, "access").unwrap();
            writeln!(file, "accident").unwrap();
            writeln!(file, "account").unwrap();
            writeln!(file, "accuse").unwrap();
            writeln!(file, "achieve").unwrap();
            writeln!(file, "acid").unwrap();
            writeln!(file, "acoustic").unwrap();
            writeln!(file, "acquire").unwrap();
            writeln!(file, "across").unwrap();
            writeln!(file, "act").unwrap();
            writeln!(file, "action").unwrap();
            writeln!(file, "actor").unwrap();
            writeln!(file, "actress").unwrap();
            writeln!(file, "actual").unwrap();
            writeln!(file, "adapt").unwrap();
        }

        let wl = WordList::default();
        let collector = FileSeedCollector::new(wl, file_path.clone());

        // Collect the seed
        let result = collector.collect();
        assert!(result.is_ok(), "Expected to collect seed successfully");

        let seed = result.unwrap();
        // Check that the seed has 24 indices
        assert_eq!(seed.length, 24);
    }

    #[test]
    fn test_file_seed_collector_missing_file() {
        // Attempt to open a file that doesn't exist
        let wl = WordList::default();
        let path = PathBuf::from("non_existent_file.txt");
        let collector = FileSeedCollector::new(wl, path);

        let result = collector.collect();
        assert!(result.is_err(), "Expected an error for non-existent file");
    }
}
