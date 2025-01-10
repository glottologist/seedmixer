use crate::errors::ShredError;
use file_shred::ShredConfig;
use std::path::PathBuf;

/// A trait defining how files should be shredded.
pub trait ShredFile {
    fn shred(&self, file_paths: Vec<PathBuf>) -> Result<(), ShredError>;
}

/// The core struct that implements file shredding with a configurable number of passes.
pub struct Shred {
    passes: u32,
}

impl Shred {
    /// Creates a new `Shred` instance with the given number of passes.
    pub fn new(passes: u32) -> Self {
        Self { passes }
    }
}

/// Provides a secure default of 7 overwrite passes.
impl Default for Shred {
    fn default() -> Self {
        Self { passes: 7u32 }
    }
}

impl ShredFile for Shred {
    fn shred(&self, file_paths: Vec<PathBuf>) -> Result<(), ShredError> {
        // Create a `ShredConfig` with the user-specified number of passes.
        let config = ShredConfig::non_interactive(
            file_paths,
            file_shred::Verbosity::Normal,
            false,
            self.passes,
            self.passes,
        );

        file_shred::shred(&config).map_err(ShredError::UnableToShredFile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shred_default_passes() {
        let shredder = Shred::default();
        // We expect 7 passes by default.
        assert_eq!(shredder.passes, 7);
    }

    #[test]
    fn test_shred_custom_passes() {
        let shredder = Shred::new(3);
        assert_eq!(shredder.passes, 3);
    }

    #[test]
    fn test_shred_operation() {
        let shredder = Shred::default();

        // Create a temp file for testing.
        let temp_dir = std::env::temp_dir();
        let test_file = temp_dir.join("shred_testfile.tmp");

        // Write something to the file.
        std::fs::write(&test_file, b"Important data").expect("Failed to write test data");

        // Shred the file.
        let result = shredder.shred(vec![test_file.clone()]);

        // We expect this to succeed
        assert!(result.is_ok(), "Shredding should succeed.");

        // Check if the file was removed or truncated (depends on how file_shred works).
        assert!(
            !test_file.exists(),
            "Shredded file should be removed or overwritten."
        );
    }
}
