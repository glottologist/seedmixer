use super::EncryptedSecretShare;
use crate::{
    crypt::{Crypt, EncSettings},
    errors::StorageError,
    mix::SecretShare,
};
use num_bigint::BigInt;
use std::{collections::HashMap, fs::File};

/// A trait for storing multiple `SecretShare`s. Implementations might encrypt them, or store them in plaintext.
pub trait ShareStorer {
    /// Stores a collection of shares indexed by `BigInt`.
    /// Each share is typically serialized to JSON in a file.
    fn store(&self, shares: HashMap<BigInt, SecretShare>) -> Result<(), StorageError>;
}

/// Encrypts and stores each `SecretShare` to a JSON file, plus writes an associated key file.
pub struct EncryptedShareStorer {
    /// If provided, this string will prefix each share and key file. Otherwise, "secret" is used.
    override_file_name: Option<String>,
    //If provided, this uses the phrase for secret key
    enc_settings: EncSettings,
}

impl EncryptedShareStorer {
    /// Creates a new `EncryptedShareStorer` with an optional file prefix.
    pub fn new(override_file_name: Option<String>, enc_settings: EncSettings) -> Self {
        Self {
            override_file_name,
            enc_settings,
        }
    }
}

impl ShareStorer for EncryptedShareStorer {
    fn store(&self, shares: HashMap<BigInt, SecretShare>) -> Result<(), StorageError> {
        // Generate a new keypair on each store call.
        let crypt = Crypt::new(self.enc_settings.clone())?;
        // Determine the file prefix
        let file_name_prefix = match &self.override_file_name {
            Some(n) => format!("{}_enc_share", n.clone()),
            None => "secret_enc_share".to_string(),
        };

        // Iterate over the shares
        for (index, secret_share) in &shares {
            let mut enc_shares: Vec<Vec<u8>> = Vec::new();

            // Encrypt each BigInt share
            for share in &secret_share.shares {
                // We only need the byte slice from `to_bytes_le()` - all values would be positive
                let bytes = share.to_bytes_le();
                let enc = crypt.encrypt(bytes.1).map_err(StorageError::Cryptography)?;
                enc_shares.push(enc)
            }

            // Create the `EncryptedSecretShare`
            let enc_share = EncryptedSecretShare {
                index: index.clone(),
                threshold: secret_share.threshold.clone(),
                total: secret_share.total.clone(),
                shares: enc_shares,
            };

            // Write JSON to a file named like "prefix_share_index_of_total.json"
            let file_name = format!(
                "{}_{}_of_{}.json",
                &file_name_prefix, enc_share.index, enc_share.total
            );
            let file = File::create(file_name).map_err(StorageError::UnableToReadWriteFile)?;
            // Serialize the encrypted share to JSON
            let _ = serde_json::to_writer(file, &enc_share);
        }

        match self.enc_settings {
            EncSettings::Key(_) => {
                // If using enc key rather than phrase, write the encryption key as hex to a separate JSON file
                let key_file_name = format!("{}_key.json", &file_name_prefix);
                let keyfile =
                    File::create(key_file_name).map_err(StorageError::UnableToReadWriteFile)?;
                let key_hex = hex::encode(crypt.secret_key.secret_bytes());
                let _ = serde_json::to_writer(keyfile, &key_hex);
            }
            EncSettings::Phrase(_) => (),
        }

        Ok(())
    }
}

/// Stores shares in plaintext JSON form.
pub struct PlainShareStorer {
    /// If provided, this string will prefix each share file. Otherwise, "secret" is used.
    override_file_name: Option<String>,
}

impl PlainShareStorer {
    /// Creates a new `PlainShareStorer` with an optional file prefix.
    pub fn new(override_file_name: Option<String>) -> Self {
        Self { override_file_name }
    }
}

impl Default for PlainShareStorer {
    fn default() -> Self {
        Self::new(None)
    }
}

impl ShareStorer for PlainShareStorer {
    fn store(&self, shares: HashMap<BigInt, SecretShare>) -> Result<(), StorageError> {
        let file_name_prefix = match &self.override_file_name {
            Some(n) => n.clone(),
            None => "secret".to_string(),
        };

        for (index, secret_share) in &shares {
            let file_name = format!(
                "{}_share_{}_of_{}.json",
                &file_name_prefix, index, secret_share.total
            );

            // Open the file
            let file = File::create(&file_name).map_err(StorageError::UnableToReadWriteFile)?;
            // Serialize the share in plaintext
            let _ = serde_json::to_writer(file, &secret_share);
            println!("Share stored {}", &file_name);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mix::SecretShare;
    use num_bigint::BigInt;
    use std::fs;

    #[test]
    fn test_plain_share_storer_default_prefix() {
        // Build sample shares
        let mut shares_map = HashMap::new();
        shares_map.insert(
            BigInt::from(1),
            SecretShare {
                index: BigInt::from(1),
                threshold: BigInt::from(2),
                total: BigInt::from(5),
                shares: vec![BigInt::from(42)],
            },
        );

        // Create storer
        let storer = PlainShareStorer::default();

        // Store
        let result = storer.store(shares_map);
        assert!(result.is_ok(), "{}", format!("{:?}", result));

        // We expect "secret_share_1_of_5.json" + check contents
        let contents = fs::read_to_string("secret_share_1_of_5.json").unwrap();
        println!("{}", contents);
        assert!(contents.contains("\"index\":[1,[1]]"));
        assert!(contents.contains("\"threshold\":[1,[2]]"));
        assert!(contents.contains("\"total\":[1,[5]]"));
        assert!(contents.contains("42"));

        //Clean up
        for file in vec!["secret_share_1_of_5.json"] {
            let _ = fs::remove_file(file).expect("We should be able to delete the file");
        }
    }

    #[test]
    fn test_plain_share_storer_custom_prefix() {
        let mut shares_map = HashMap::new();
        shares_map.insert(
            BigInt::from(1),
            SecretShare {
                index: BigInt::from(1),
                threshold: BigInt::from(2),
                total: BigInt::from(5),
                shares: vec![BigInt::from(42)],
            },
        );

        // Provide a custom prefix "my_prefix"
        let storer = PlainShareStorer::new(Some("my_prefix".to_string()));
        let result = storer.store(shares_map);
        assert!(result.is_ok(), "{}", format!("{:?}", result));

        // We expect "my_prefix_share_1_of_5.json"
        let contents = fs::read_to_string("my_prefix_share_1_of_5.json").unwrap();
        println!("{}", contents);
        assert!(contents.contains("\"index\":[1,[1]]"));

        let _ = fs::remove_file("my_prefix_share_1_of_5.json")
            .expect("We should be able to delete the file");
    }
}
