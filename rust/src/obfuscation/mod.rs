use crate::words::WORDLISTSIZE;
use std::collections::HashMap;

use crate::errors::ObfuscationSeedError;
use crate::seed::Seed;
use num_modular::{ModularCoreOps, ModularPow};

/// Structure representing a PIN, a wrapper around a vector of `u16`.
#[derive(Debug, Clone)]
pub struct Pin(pub Vec<u16>);

impl Pin {
    /// Returns the length of the PIN and a mapping of PIN index to value.
    /// Useful for indexing and avoiding repetitive calculations.
    fn get_pin_length_and_map(&self) -> (u16, HashMap<u16, u16>) {
        let pin_length = self.0.len() as u16;
        // Maps each index to the corresponding PIN digit value.
        let pin_map: HashMap<u16, u16> = self
            .0
            .iter()
            .enumerate()
            .map(|(index, &value)| (index as u16, value))
            .collect();
        (pin_length, pin_map)
    }
}

/// Represents an obfuscated seed with indices modified based on the PIN.
#[derive(Debug)]
pub struct ObfuscatedSeed {
    pub indices: Vec<u16>, // Obfuscated indices
    pub length: u16,       // Number of indices
}

/// Trait for shifting a seed digit by a given PIN digit.
pub trait Shifter {
    /// Shifts the seed digit by the PIN digit, using a modulus to wrap around.
    fn shift(&self, pin_digit: u16, seed_digit: u16) -> u16;
}

/// Implementation of Shifter using power-of-two shifting logic.
#[derive(Debug)]
pub struct PowerOfTwoShift {}
impl Shifter for PowerOfTwoShift {
    fn shift(&self, pin_digit: u16, seed_digit: u16) -> u16 {
        let modulus = WORDLISTSIZE + 1;
        let shift = 2u16.powm(pin_digit, &modulus); // Modular exponentiation
        seed_digit.addm(shift, &modulus)
    }
}

/// Reverse shifting logic for power-of-two shifting.
#[derive(Debug)]
pub struct PowerOfTwoUnshift {}
impl Shifter for PowerOfTwoUnshift {
    fn shift(&self, pin_digit: u16, seed_digit: u16) -> u16 {
        let modulus = WORDLISTSIZE + 1;
        let shift = 2u16.powm(pin_digit, &modulus); // Modular exponentiation
        seed_digit.subm(shift, &modulus)
    }
}

/// Trait for obfuscating seeds using a specific shifting strategy.
pub trait Obfuscator {
    /// Obfuscates the seed using the provided shifter and PIN.
    fn obfuscate(
        &self,
        pin: &Pin,
        shifter: impl Shifter,
    ) -> Result<ObfuscatedSeed, ObfuscationSeedError>
    where
        Self: Sized;
}

/// Trait for deobfuscating obfuscated seeds back into original seeds.
pub trait Deobfuscator {
    fn deobfuscate(&self, pin: &Pin, shifter: impl Shifter) -> Result<Seed, ObfuscationSeedError>
    where
        Self: Sized;
}

// Implement Obfuscator for the Seed structure.
impl Obfuscator for Seed {
    fn obfuscate(
        &self,
        pin: &Pin,
        shifter: impl Shifter,
    ) -> Result<ObfuscatedSeed, ObfuscationSeedError> {
        let (pin_length, pin_map) = pin.get_pin_length_and_map();
        let mut obfuscated = Vec::<u16>::new();
        for (index, &seed_digit) in self.indices.iter().enumerate() {
            let pin_index = (index as u16) % pin_length;
            let pin_value_at_index = pin_map
                .get(&pin_index)
                .ok_or(ObfuscationSeedError::InvalidPinIndex)?;
            let shifted_digit = shifter.shift(*pin_value_at_index, seed_digit);
            obfuscated.push(shifted_digit);
        }

        Ok(ObfuscatedSeed {
            length: obfuscated.len() as u16,
            indices: obfuscated,
        })
    }
}

// Implement Deobfuscator for the ObfuscatedSeed structure.
impl Deobfuscator for ObfuscatedSeed {
    fn deobfuscate(&self, pin: &Pin, shifter: impl Shifter) -> Result<Seed, ObfuscationSeedError> {
        let (pin_length, pin_map) = pin.get_pin_length_and_map();
        let mut obfuscated = Vec::<u16>::new();
        for (i, v) in self.indices.iter().enumerate() {
            let pin_index = (i as u16) % pin_length;
            let pin_value_at_index = pin_map[&pin_index];
            let obs = shifter.shift(pin_value_at_index, *v);
            obfuscated.push(obs);
        }
        Ok(Seed {
            length: obfuscated.len() as u16,
            indices: obfuscated.clone(),
        })
    }
}

// Test module to validate the functionality of the shifting and obfuscating methods.
#[cfg(test)]
mod tests {
    use proptest::{prop_assert, prop_assert_eq, prop_assert_ne, prop_oneof};

    use {
        super::*,
        proptest::{
            prelude::{ProptestConfig, Strategy},
            proptest,
        },
    };

    // A strategy to generate unique vector seeds conforming to specific criteria.
    fn seed_strategy() -> impl Strategy<Value = Vec<u16>> {
        proptest::collection::vec(1u16..=WORDLISTSIZE, 1..=24)
            .prop_filter("Must have lengths 12,16 or 24", |s| {
                s.len() == 12 || s.len() == 16 || s.len() == 24
            })
    }
    // A strategy to generate 4 digit pins.
    fn pin_strategy() -> impl Strategy<Value = Pin> {
        prop_oneof![
            proptest::collection::vec(1u16..10u16, 4).prop_map(|p| Pin(p)),
            proptest::collection::vec(1u16..10u16, 6).prop_map(|p| Pin(p)),
            proptest::collection::vec(1u16..10u16, 12).prop_map(|p| Pin(p)),
        ]
    }

    // Property tests to ensure the integrity of the obfuscation and deobfuscation processes.
    proptest! {
           #![proptest_config(ProptestConfig::with_cases(5000))]

           #[test]
           fn test_power_of_two_mask_prop(pin in pin_strategy(),seed in seed_strategy()) {
           let s = Seed::new(seed)?;
           let obs=s.obfuscate(&pin,PowerOfTwoShift{})?;
           let s_vec:Vec<u16> = s.indices.clone().into();
           prop_assert_ne!(s_vec,obs.indices.clone(),"Seed indices should be different after obfuscation");
           let unobs=obs.deobfuscate(&pin,PowerOfTwoUnshift{})?;
           prop_assert_eq!(&s.indices,&unobs.indices, "Deobsfuscation should regain original seed");
           }

    /// Tests that obfuscating and deobfuscating a seed returns the original seed.
           #[test]
           fn test_obfuscate_and_deobfuscate(pin in pin_strategy(), seed in seed_strategy()) {
               let seed_instance = Seed::new(seed.clone()).expect("Failed to create Seed");
               let obfuscated = seed_instance.obfuscate(&pin, PowerOfTwoShift{})
                   .expect("Obfuscation failed");
               let deobfuscated = obfuscated.deobfuscate(&pin, PowerOfTwoUnshift{})
                   .expect("Deobfuscation failed");

               prop_assert_eq!(
                   seed_instance.indices,
                   deobfuscated.indices,
                   "Deobfuscation did not return the original seed"
               );
           }



    /// Tests that obfuscated indices differ from the original seed indices.
           #[test]
           fn test_obfuscated_differs_from_original(pin in pin_strategy(), seed in seed_strategy()) {
               let seed_instance = Seed::new(seed.clone()).expect("Failed to create Seed");
               let obfuscated = seed_instance.obfuscate(&pin, PowerOfTwoShift{})
                   .expect("Obfuscation failed");

               let original_indices: Vec<u16> = seed_instance.indices.into();
               prop_assert_ne!(
                   original_indices,
                   obfuscated.indices,
                   "Obfuscated seed should differ from the original seed"
               );
           }

           #[test]
           fn test_stress_obfuscation(pin in pin_strategy(), seed in seed_strategy()) {
               let seed_instance = Seed::new(seed.clone()).expect("Failed to create Seed");
               let obfuscated = seed_instance.obfuscate(&pin, PowerOfTwoShift{})
                   .expect("Obfuscation failed");

               // Ensure obfuscated indices are valid and within range.
               for &obf_digit in &obfuscated.indices {
                   prop_assert!(obf_digit <= WORDLISTSIZE, "Obfuscated digit out of range");
               }
           }
       }
}
