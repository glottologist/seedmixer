use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use num_bigint::BigInt;

use crate::{
    crypt::{Crypt, EncSettings},
    errors::{CollectionError, StorageError},
    mix::SecretShare,
    storage::EncryptedSecretShare,
};

/// A trait representing any type that can collect `SecretShare` data
/// from a particular source (files, encrypted files, etc.).
pub trait ShareCollector {
    fn collect(&self) -> Result<HashMap<BigInt, SecretShare>, CollectionError>;
}

/// Collects `SecretShare`s. Returns an error if something goes wrong,
/// such as an invalid file, parse failure, or decryption error.
pub struct FileShareCollector {
    paths: Vec<PathBuf>,
}

/// Collects unencrypted `SecretShare`s from one or more file paths.
impl FileShareCollector {
    /// Creates a new collector given a list of file paths.
    pub fn new(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }
}

impl ShareCollector for FileShareCollector {
    fn collect(&self) -> Result<HashMap<BigInt, SecretShare>, CollectionError> {
        let share_length = self.paths.len();
        if share_length == 0 {
            return Err(CollectionError::NotEnoughSharesProvided(share_length));
        }

        let mut shares: HashMap<BigInt, SecretShare> = HashMap::new();

        // Loop over the file paths
        for path in &self.paths {
            // Attempt to open each file, returning an IO error on failure.
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            // Deserialize the file's contents into a SecretShare struct
            let share: SecretShare =
                serde_json::from_reader(reader).map_err(CollectionError::UnableToParseFile)?;

            // Insert into the map (overwrites if a duplicate index is found).
            shares.insert(share.index.clone(), share);
        }

        // Check if we got exactly one share per file
        if shares.len() == self.paths.len() {
            Ok(shares)
        } else {
            Err(CollectionError::UnableToCollectSharesFromFiles)
        }
    }
}

/// Collects encrypted `SecretShare`s from multiple files, using a hex-encoded decryption key.
pub struct EncryptedFileShareCollector {
    paths: Vec<PathBuf>,
    decryption_settings: EncSettings,
}

impl EncryptedFileShareCollector {
    /// Creates a new encrypted collector with the specified paths and hex-encoded key.
    pub fn new(paths: Vec<PathBuf>, decryption_settings: EncSettings) -> Self {
        Self {
            paths,
            decryption_settings,
        }
    }
}

impl ShareCollector for EncryptedFileShareCollector {
    fn collect(&self) -> Result<HashMap<BigInt, SecretShare>, CollectionError> {
        // Create a `Crypt` instance from the secret key
        let crypt = Crypt::new(self.decryption_settings.clone())?;
        let mut shares: HashMap<BigInt, SecretShare> = HashMap::new();

        // Read each file
        for path in &self.paths {
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            // Deserialize the encrypted secret share
            let enc_share: EncryptedSecretShare =
                serde_json::from_reader(reader).map_err(CollectionError::UnableToParseFile)?;

            let mut enc_shares: Vec<BigInt> = Vec::new();

            // Decrypt each portion of the share
            for e in enc_share.shares.into_iter() {
                let enc = crypt.decrypt(e).map_err(StorageError::Cryptography)?;

                // Convert the decrypted bytes to a BigInt
                let b_int = BigInt::from_bytes_le(num_bigint::Sign::Plus, &enc);
                enc_shares.push(b_int)
            }

            // Construct the decrypted `SecretShare`
            let share = SecretShare {
                index: enc_share.index,
                threshold: enc_share.threshold.clone(),
                total: enc_share.total.clone(),
                shares: enc_shares,
            };

            // Insert the share into our map
            shares.insert(share.index.clone(), share);
        }

        // Verify that each file resulted in a unique share
        if shares.len() == self.paths.len() {
            Ok(shares)
        } else {
            Err(CollectionError::UnableToCollectSharesFromFiles)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mix::SecretShare;
    use num_bigint::BigInt;
    use std::fs::File;
    use tempfile::tempdir; // or tempfile::TempDir

    #[test]
    fn test_file_share_collector_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("share1.json");

        // Prepare a SecretShare
        let secret_share = SecretShare {
            index: BigInt::from(1),
            threshold: 2.into(),
            total: 3.into(),
            shares: vec![BigInt::from(42)],
        };

        // Write the share as JSON
        {
            let mut file = File::create(&file_path).unwrap();
            serde_json::to_writer(&mut file, &secret_share).unwrap();
        }

        // Create collector
        let collector = FileShareCollector::new(vec![file_path]);
        let result = collector.collect();
        assert!(result.is_ok(), "Expected successful share collection");
        let shares = result.unwrap();
        assert_eq!(shares.len(), 1);
        let retrieved = shares.get(&BigInt::from(1)).unwrap();
        assert_eq!(retrieved.index, BigInt::from(1));
        assert_eq!(retrieved.threshold, BigInt::from(2));
        assert_eq!(retrieved.total, BigInt::from(3));
        assert_eq!(retrieved.shares[0], BigInt::from(42));
    }

    #[test]
    fn test_file_share_collector_file_not_found() {
        let collector = FileShareCollector::new(vec![PathBuf::from("nonexistent.json")]);
        let result = collector.collect();
        assert!(result.is_err());
    }
}
