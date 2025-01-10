use crate::errors::CryptError;
use secp256k1::{generate_keypair, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

/// A struct for storing the optional encryption phrase
#[derive(Clone)]
pub enum EncSettings {
    Phrase(String),
    Key(Option<String>),
}

impl EncSettings {
    pub fn from_phrase(phrase: String) -> EncSettings {
        EncSettings::Phrase(phrase)
    }
    pub fn from_key_opt(key: Option<String>) -> EncSettings {
        EncSettings::Key(key)
    }
}

/// A simple struct that holds both a secp256k1 public key and secret key.
/// This can be used for ECIES-style encryption and decryption.
pub struct Crypt {
    pub pub_key: PublicKey,
    pub secret_key: SecretKey,
}

impl Default for Crypt {
    fn default() -> Self {
        Self::new(EncSettings::Key(None)).unwrap()
    }
}

impl Crypt {
    /// Creates a new keypair using a random number generator.
    pub fn new(enc_settings: EncSettings) -> Result<Self, CryptError> {
        use EncSettings::*;
        match enc_settings {
            Phrase(phrase) => Self::from_enc_phrase(phrase),
            Key(key_opt) => match key_opt {
                Some(k) => {
                    // Decode the hex string to bytes
                    let key = hex::decode(k).map_err(CryptError::UnableToConvertKeyFromHex)?;

                    // Convert the bytes into a 32-byte array
                    let key_bytes: &[u8; 32] = &key
                        .as_slice()
                        .try_into()
                        .map_err(CryptError::UnableToGetSecretKeyFromBytesSlice)?;
                    Self::from_secret_key_bytes(key_bytes)
                }
                None => {
                    let (secret_key, pub_key) = generate_keypair(&mut rand::thread_rng());

                    Ok(Self {
                        pub_key,
                        secret_key,
                    })
                }
            },
        }
    }

    pub fn from_enc_phrase(enc_phrase: String) -> Result<Self, CryptError> {
        // Hash the sentence using SHA-256
        let mut hasher = Sha256::new();
        hasher.update(enc_phrase.as_bytes());
        let hash_result = hasher.finalize();
        Self::from_secret_key_bytes(&hash_result.into())
    }

    /// Creates a new `Crypt` from existing secret key bytes.
    /// Returns an error if the slice cannot produce a valid `SecretKey`.
    pub fn from_secret_key_bytes(bytes: &[u8; 32]) -> Result<Self, CryptError> {
        let secret_key: SecretKey =
            SecretKey::from_slice(bytes).map_err(CryptError::UnableToGetSecretKeyFromBytes)?;
        // Create a Secp256k1 context.
        let secp = Secp256k1::new();
        let pub_key = PublicKey::from_secret_key(&secp, &secret_key);
        Ok(Self {
            pub_key,
            secret_key,
        })
    }

    /// Encrypts arbitrary data (`to_encrypt`) using this crypt's public key.
    /// Returns the ciphertext or an error.
    pub fn encrypt(&self, to_encrypt: Vec<u8>) -> Result<Vec<u8>, CryptError> {
        let pubkey_unc = self.pub_key.serialize_uncompressed();
        // The ECIES library returns a result; convert any error into `CryptError`.
        ecies::encrypt(&pubkey_unc, &to_encrypt)
            .map_err(|e| CryptError::UnableToEncryptDecrypt(e.to_string()))
    }

    /// Decrypts data (`to_decrypt`) using this crypt's secret key.
    /// Returns the plaintext or an error.
    pub fn decrypt(&self, to_decrypt: Vec<u8>) -> Result<Vec<u8>, CryptError> {
        ecies::decrypt(&self.secret_key.secret_bytes(), &to_decrypt)
            // `.map(|e| e.to_vec())` to convert from `&[u8]` to `Vec<u8>`.
            .map(|e| e.to_vec())
            .map_err(|e| CryptError::UnableToEncryptDecrypt(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use secp256k1::SecretKey;
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_from_secret_key_bytes_error() {
        // Suppose all zeros is invalid for a secp256k1 key,
        // or you can pick something else known to be invalid
        let bytes = [0u8; 32];
        let result = Crypt::from_secret_key_bytes(&bytes);
        assert!(result.is_err(), "Expected an error on invalid bytes");
    }

    #[test]
    fn test_encrypt_and_decrypt_empty_message() {
        let crypt = Crypt::new(EncSettings::Key(None)).unwrap();
        let message = Vec::new();
        let encrypted = crypt.encrypt(message.clone()).unwrap();
        let decrypted = crypt.decrypt(encrypted).unwrap();
        assert_eq!(
            decrypted, message,
            "Empty message should remain empty after encryption/decryption"
        );
    }

    #[test]
    fn test_encrypt_and_decrypt() {
        let crypt = Crypt::default();

        // Generate a message to encrypt
        let message = b"Hello, Rust!".to_vec();

        // Encrypt the message
        let encrypted = crypt.encrypt(message.clone());
        assert!(encrypted.is_ok());
        let encrypted_data = encrypted.unwrap();

        // Decrypt the message
        let decrypted = crypt.decrypt(encrypted_data).unwrap();

        // Assert that the decrypted message matches the original
        assert_eq!(decrypted, message);
    }

    #[test]
    fn test_encrypt_and_decrypt_with_stored_key() {
        let crypt = Crypt::default();

        // Generate a message to encrypt
        let message = b"Hello, Rust!".to_vec();

        let file_name = "secret_key.json";
        let keyfile = File::create(file_name).unwrap();
        let _ = serde_json::to_writer(keyfile, &crypt.secret_key);
        let file = File::open(file_name).unwrap();
        let reader = BufReader::new(file);
        let key: SecretKey = serde_json::from_reader(reader).unwrap();
        println!("recovered key {:?}", key);

        // Encrypt the message
        let encrypted = crypt.encrypt(message.clone());
        assert!(encrypted.is_ok());
        let encrypted_data = encrypted.unwrap();

        // Decrypt the message
        let decrypt = Crypt::from_secret_key_bytes(&key.secret_bytes()).unwrap();
        let decrypted = decrypt.decrypt(encrypted_data).unwrap();

        // Assert that the decrypted message matches the original
        assert_eq!(decrypted, message);
        let _ = std::fs::remove_file(file_name).expect("We should be able to delete the file");
    }

    proptest! {
        // This strategy generates random vectors of up to length 256 of random bytes.
        #[test]
        fn encrypt_decrypt_random_data(ref data in proptest::collection::vec(any::<u8>(), 0..256)) {
            let crypt = Crypt::new(EncSettings::Key(None)).unwrap();

            // Encrypt the random data
            let encrypted_data = crypt.encrypt(data.clone());
            prop_assert!(encrypted_data.is_ok());

            let decrypted_data = crypt.decrypt(encrypted_data.unwrap());
            prop_assert!(decrypted_data.is_ok());

            // The decrypted data should match the original
            prop_assert_eq!(decrypted_data.unwrap(), data.clone());
        }
    }
}
