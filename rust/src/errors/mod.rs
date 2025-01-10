use image::ImageError;
use num_bigint::ParseBigIntError;
use std::{array::TryFromSliceError, io, path::PathBuf};
use thiserror::Error;

// Define custom error types for SeedMixer-related operations
#[derive(Error, Debug)]
pub enum SeedMixerError {
    #[error("Shamir error: {0}")]
    Shamir(#[from] ShamirError),
    #[error("Shred error: {0}")]
    Shred(#[from] ShredError),
    #[error("File error: {0}")]
    File(#[from] FileError),
    #[error("Collection error: {0}")]
    Collection(#[from] CollectionError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Ascii error: {0}")]
    Ascii(#[from] AsciiError),
    #[error("Command line arguments error: {0}")]
    CommandLineArgs(#[from] CommandLineArgsError),
    #[error("Seed phrase error: {0}")]
    SeedError(#[from] SeedPhraseError),
    #[error("Obfucation error: {0}")]
    ObfuscationError(#[from] ObfuscationSeedError),
    #[error("Incorrect number of shares error")]
    IncorrectNumberOfSharesGenerated,
    #[error("Not enough threshold shares provided error")]
    NotEnoughThresholdSharesProvided,
    #[error("Too many threshold shares provided error")]
    TooManyThresholdSharesProvided,
    #[error("Incorrect number of shares provided: threshold {0}, total {1}")]
    IncorrectNumberOfSharesProvided(usize, usize),
}

// Define error type for file operation issues
#[derive(Error, Debug)]
pub enum FileError {
    #[error("Unable to read file: {0}")]
    UnableToReadWriteFile(#[from] io::Error),
    #[error("Unable to parse file: {0}")]
    UnableToParseFile(#[from] serde_json::Error),
    #[error("Cryptography error: {0}")]
    Cryptography(#[from] CryptError),
    #[error("Unable to collect shares from files")]
    UnableToCollectSharesFromFiles,
}

// Define error type for ASCII art generation issues
#[derive(Error, Debug)]
pub enum AsciiError {
    #[error("Unable to load ascii art image from memory: {0}")]
    UnableToLoadAsciiArtImageFromMemory(#[from] ImageError),
}

// Define error type for command line argument validation issues
#[derive(Error, Debug)]
pub enum CommandLineArgsError {
    #[error("Pin is not a valid number: {0}")]
    PinIsNotAValidNumber(#[from] std::num::ParseIntError),
    #[error("Pin should not contain any zeros")]
    PinShouldNotContainZero,
    #[error("File path is not valid")]
    FilePathIsNotValid,
}

// Define error type for seed phrase validation and processing issues
#[derive(Error, Debug)]
pub enum SeedPhraseError {
    #[error(
        "Number of seed entries are incorrect; seeds should be 12, 16 or 24 words in length - {0}"
    )]
    SeedPhraseLengthIsIncorrect(usize),
    #[error(
        "Seed phase indices can only fall between 1 and 2048 as defined in the BIP-39 word lists"
    )]
    SeedPhraseIndexIsOutsideTheBoundsOfTheWordList,
    #[error("Cannot read seed phrase file contents from given path")]
    CannotReadContentsOfSeedPhraseFileFromPath,
    #[error("Cannot read single word from seed phrase file")]
    CannotReadWordFromSeedPhraseFile,
    #[error("Unable to determine input from terminal: {0}")]
    UnableToDetermineTerminalInput(#[from] std::io::Error),
}

// Define error type for obfuscation-specific issues
#[derive(Error, Debug)]
pub enum ObfuscationSeedError {
    #[error("The pin length is of an incorrect size related to the seed length")]
    PinLengthIsIncorrect,
    #[error("The index of the pin is incorrect")]
    InvalidPinIndex,
}

// Define error type for encryption/decryption errors
#[derive(Error, Debug)]
pub enum CryptError {
    #[error("Unable to encrypt/decrypt data")]
    UnableToEncryptDecrypt(String),
    #[error("No public key available to encrypt")]
    NoPublicKeyAvailableToEncrypt,
    #[error("Unable to get secret key from bytes")]
    UnableToGetSecretKeyFromBytes(#[from] secp256k1::Error),
    #[error("Unable to get secret key from bytes {0}")]
    UnableToGetSecretKeyFromBytesSlice(#[from] TryFromSliceError),
    #[error("Unable to convert key from hex {0}")]
    UnableToConvertKeyFromHex(#[from] hex::FromHexError),
}

// Define error type for encryption/decryption errors
#[derive(Error, Debug)]
pub enum CollectionError {
    #[error("Cryptography error: {0}")]
    Cryptography(#[from] CryptError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Unable to read seed from file")]
    UnableToReadSeedFromFile(PathBuf),
    #[error("Unable to read file: {0}")]
    UnableToReadWriteFile(#[from] io::Error),
    #[error("Unable to parse file: {0}")]
    UnableToParseFile(#[from] serde_json::Error),
    #[error("Unable to collect shares from files")]
    UnableToCollectSharesFromFiles,
    #[error("Not enough shares provided {0}")]
    NotEnoughSharesProvided(usize),
}
// Define error type for encryption/decryption errors
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Unable to read file: {0}")]
    UnableToReadWriteFile(#[from] io::Error),
    #[error("Cryptography error: {0}")]
    Cryptography(#[from] CryptError),
}

#[derive(Error, Debug)]
pub enum ShredError {
    #[error("Unable to shred file: {0}")]
    UnableToShredFile(String),
}

#[derive(Error, Debug, Clone)]
pub enum ShamirError {
    #[error("Unable to parse big int: {0}")]
    UnableToParseBigInt(#[from] ParseBigIntError),
    #[error("Unable to convert biguint to bigint")]
    UnableToConvertBigUintToBigInt,
}
