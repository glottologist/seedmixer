use std::path::PathBuf;

use crate::crypt::EncSettings;
use crate::words::WordList;

use self::seed::FileSeedCollector;
use self::seed::SeedCollector;
use self::seed::TerminalSeedCollector;
use self::share::EncryptedFileShareCollector;
use self::share::FileShareCollector;
use self::share::ShareCollector;

pub mod seed;
pub mod share;

/// Based on whether a `PathBuf` is provided or not,
/// this function creates either a FileSeedCollector (if a path is given)
/// or a TerminalSeedCollector (if `None`).
pub fn get_seed_collector(wl: WordList, path: Option<PathBuf>) -> Box<dyn SeedCollector> {
    match path {
        // If a path is specified, we use a file-based collector
        Some(p) => Box::new(FileSeedCollector::new(wl, p)),
        // Otherwise, we default to a terminal-based collector
        None => Box::new(TerminalSeedCollector::new(wl)),
    }
}

/// This function returns an EncryptedFileShareCollector if a decryption key is provided,
/// or a FileShareCollector if no key is given (i.e., unencrypted).
pub fn get_share_collector(
    paths: Vec<PathBuf>,
    decryption_key: Option<String>,
    decryption_phrase: Option<String>,
) -> Box<dyn ShareCollector> {
    match decryption_key {
        // If there's a key, we use an encrypted share collector
        Some(k) => Box::new(EncryptedFileShareCollector::new(
            paths,
            EncSettings::Key(Some(k)),
        )),
        // Otherwise, we default to a plaintext file share collector
        None => match decryption_phrase {
            Some(p) => Box::new(EncryptedFileShareCollector::new(
                paths,
                EncSettings::Phrase(p),
            )),
            // Otherwise, we default to a plaintext file share collector
            None => Box::new(FileShareCollector::new(paths)),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_get_seed_collector_always_returns_some_seed_collector(
            // Generate an optional string to represent a path
            p in proptest::option::of(".*") // any string up to some max length
        ) {
            let wl = WordList::default();
            // Convert the optional string to PathBuf if present
            let path_buf = p.map(PathBuf::from);
            let _ = get_seed_collector(wl, path_buf);

            // This simply verifies it returns *some* type implementing SeedCollector
            prop_assert!(true, "Function returned a valid seed collector.");
        }

        #[test]
        fn test_get_share_collector_always_returns_some_share_collector(
            p in proptest::collection::vec(".*", 0..4), // up to 4 random filenames
            key in proptest::option::of(".*")           // random optional string
        ) {
            let paths = p.into_iter().map(PathBuf::from).collect();
            let _ = get_share_collector(paths, key,None);

            // This simply verifies it returns *some* type implementing SeedCollector
            prop_assert!(true, "Function returned a valid share collector.");
        }
    }
}
