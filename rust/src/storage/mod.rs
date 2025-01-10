use num_bigint::BigInt;
use serde_derive::{Deserialize, Serialize};

use crate::{crypt::EncSettings, words::WordList};

use self::{
    seed::{FileSeedStorer, SeedStorer, TerminalSeedStorer},
    share::{EncryptedShareStorer, PlainShareStorer, ShareStorer},
};

pub mod seed;
pub mod share;

/// Returns a `Box<dyn ShareStorer>` implementation based on the `encrypt` flag:
/// - If `true`, uses `EncryptedShareStorer`.
/// - If `false`, uses `PlainShareStorer`.
pub fn get_share_storer(
    override_file_name: Option<String>,
    enc_settings_opt: Option<EncSettings>,
) -> Box<dyn ShareStorer> {
    match enc_settings_opt {
        Some(enc_settings_opt) => {
            // Return an encrypted share storer
            Box::new(EncryptedShareStorer::new(
                override_file_name,
                enc_settings_opt,
            ))
        }
        // Return a plain (unencrypted) share storer
        None => Box::new(PlainShareStorer::new(override_file_name)),
    }
}

/// This struct represents a secret share that has been encrypted.
/// Each entry in `shares` is a separate chunk of encrypted data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedSecretShare {
    pub index: BigInt,
    pub threshold: BigInt,
    pub total: BigInt,
    pub shares: Vec<Vec<u8>>,
}

/// Returns a `SeedStorer` implementation based on the `terminal` flag:
/// - `true` => `TerminalSeedStorer` (writes to terminal/stdout)
/// - `false` => `FileSeedStorer` (writes to a file on disk)
pub fn get_seed_storer(
    wl: WordList,
    terminal: bool,
    override_file_name: Option<String>,
) -> Box<dyn SeedStorer> {
    if terminal {
        // Return a Terminal-based storer
        return Box::new(TerminalSeedStorer::new(wl));
    }

    // Return a File-based storer
    Box::new(FileSeedStorer::new(wl, override_file_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::words::WordList;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_get_share_storer_random_bool(encrypt in any::<bool>()) {
            let enc_settings_opt = match encrypt {
                true => Some(EncSettings::Key(None)),
                false => None

            };
            let _ = get_share_storer(None, enc_settings_opt);
            prop_assert!(true);
        }

        #[test]
        fn test_get_seed_storer_random_bool(terminal in any::<bool>()) {
            let wl = WordList::default();
            let _ = get_seed_storer(wl, terminal, None);
            prop_assert!(true);
        }
    }
}
