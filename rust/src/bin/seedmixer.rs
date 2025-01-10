use clap::Parser;

use seedmixer::{
    ascii,
    cli::{Cli, Command},
    collection::{get_seed_collector, get_share_collector},
    crypt::EncSettings,
    errors::SeedMixerError,
    mix::{SeedMixer, SeedMixing},
    shred::Shred as Shredder,
    shred::ShredFile,
    storage::{get_seed_storer, get_share_storer},
    words::WordList,
};

/// Entry point for the CLI application. Returns a `Result` with `SeedMixerError` if something fails.
fn main() -> Result<(), SeedMixerError> {
    // Print ASCII art logo header (if it fails, we ignore the error, but it's not critical)
    let _ = ascii::generate_terminal_header();

    // Parse CLI arguments into our `Cli` struct
    let cli = Cli::parse();
    // Bring subcommands into scope for easier matching
    use Command::*;
    // Match on the subcommand
    match cli.command {
        CheckWordList(wordlist_args) => {
            // Create a WordList for the given language
            let wl = WordList::new(&wordlist_args.lang);
            // Parse position from string to u16
            let pos: u16 = wordlist_args.position.parse().unwrap();
            // Find the word at the specified position
            let found = wl.find(pos);
            println!("Found {} from {} wordlist", found, wl.lang);
            Ok(())
        }
        CheckWordIndex(wordlist_args) => {
            // Create a WordList for the given language
            let wl = WordList::new(&wordlist_args.lang);
            // Retrieve the word index
            let word = wordlist_args.word;
            let found = wl.get_index(&word);
            println!("Found {} from {} wordlist", found, wl.lang);
            Ok(())
        }
        Mix(mix_args) => {
            // Create a WordList for the given language
            let wl = WordList::new(&mix_args.lang);
            // Collect a seed, either from a file or from terminal input
            let seed_collector = get_seed_collector(wl, mix_args.file_path);
            let seed = seed_collector.collect()?;
            // Mix the seed into multiple shares
            let shares = SeedMixer::mix(seed, &mix_args.pin, mix_args.shares, mix_args.threshold)?;

            //Check for encryption config
            let enc_settings: Option<EncSettings> = match mix_args.encrypt {
                true => match mix_args.encryption_phrase {
                    Some(p) => Some(EncSettings::Phrase(p)),
                    None => Some(EncSettings::Key(None)),
                },
                false => mix_args.encryption_phrase.map(EncSettings::Phrase) 
            };

            // Get the appropriate share storer
            let share_storer = get_share_storer(mix_args.override_file_name, enc_settings);
            // Store the shares, possibly encrypted
            share_storer.store(shares).map_err(SeedMixerError::Storage)
        }
        Unmix(unmix_args) => {
            // Create a WordList for the given language
            let wl = WordList::new(&unmix_args.lang);
            // Collect shares, possibly encrypted, from the provided file paths
            let share_collector = get_share_collector(
                unmix_args.file_path,
                unmix_args.decryption_key,
                unmix_args.decryption_phrase,
            );
            let shares = share_collector.collect()?;
            // Reconstruct the seed from the shares
            let seed = SeedMixer::unmix(shares, &unmix_args.pin)?;
            // Decide whether to store the seed in a file or print to terminal
            let seed_storer =
                get_seed_storer(wl, unmix_args.terminal, unmix_args.override_file_name);
            let _ = seed_storer.store(seed);

            Ok(())
        }
        Shred(shred_args) => {
            // Shred the provided file paths
            let shredder = Shredder::default();
            let _ = shredder.shred(shred_args.file_path);
            Ok(())
        }
    }
}
