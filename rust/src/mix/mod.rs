use std::collections::HashMap;

use num_bigint::{BigInt, BigUint};
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

use crate::{
    errors::SeedMixerError,
    obfuscation::{
        Deobfuscator, ObfuscatedSeed, Obfuscator, Pin, PowerOfTwoShift, PowerOfTwoUnshift,
    },
    seed::Seed,
    shamir::Shamir,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretShare {
    pub index: BigInt,
    pub threshold: BigInt,
    pub total: BigInt,
    pub shares: Vec<BigInt>,
}

pub trait SeedMixing {
    fn mix(
        seed: Seed,
        pin: &Pin,
        total_shares: usize,
        threshold: usize,
    ) -> Result<HashMap<BigInt, SecretShare>, SeedMixerError>;
    fn unmix(shares: HashMap<BigInt, SecretShare>, pin: &Pin) -> Result<Seed, SeedMixerError>;
}

pub struct SeedMixer {}

impl SeedMixing for SeedMixer {
    fn mix(
        seed: Seed,
        pin: &Pin,
        total_shares: usize,
        threshold: usize,
    ) -> Result<HashMap<BigInt, SecretShare>, SeedMixerError> {
        // Validate inputs
        if threshold >= total_shares {
            return Err(SeedMixerError::IncorrectNumberOfSharesProvided(
                threshold,
                total_shares,
            ));
        }

        let shifter = PowerOfTwoShift {};
        let obfuscated = seed.obfuscate(pin, shifter)?;
        let shamir = Shamir::new(total_shares, threshold);

        // Generate Shamir shares for each obfuscated index
        let mut shares: Vec<Vec<(BigInt, BigInt)>> = Vec::new();
        for index in obfuscated.indices.clone().into_iter() {
            let share = shamir.generate_shares(&BigUint::from(index))?;
            shares.push(share);
        }

        // Initialize a share map with empty shares for each index
        let mut share_map: HashMap<BigInt, SecretShare> = HashMap::with_capacity(total_shares);

        let total = BigInt::from(total_shares);
        let th = BigInt::from(threshold);
        for i in 1..=total_shares {
            let big_i = BigInt::from(i);
            share_map.insert(
                big_i.clone(),
                SecretShare {
                    index: big_i,
                    threshold: th.clone(),
                    total: total.clone(),
                    shares: Vec::new(),
                },
            );
        }

        // Distribute shares among the map entries
        for share in shares.clone().into_iter() {
            for is in share {
                let (i, s) = is;
                let s_share = share_map.get_mut(&i).unwrap();
                s_share.shares.push(s)
            }
        }

        // Verify the length of shares
        if share_map.len() != total_shares {
            return Err(SeedMixerError::IncorrectNumberOfSharesGenerated);
        }
        for (_, share) in share_map.clone().iter() {
            if share.shares.len() != obfuscated.indices.len() {
                return Err(SeedMixerError::IncorrectNumberOfSharesGenerated);
            }
        }

        Ok(share_map)
    }

    fn unmix(shares: HashMap<BigInt, SecretShare>, pin: &Pin) -> Result<Seed, SeedMixerError> {
        println!("Unmixing");

        // Retrieve the total number of shares and threshold
        let mut total = BigInt::ZERO;
        let mut threshold = BigInt::ZERO;
        let mut seed_length = 0usize;
        if let Some((_, value)) = shares.clone().iter().next() {
            total = value.total.clone();
            threshold = value.threshold.clone();
            seed_length = value.shares.len();
        };

        let share_count = BigInt::from(shares.len());

        // Ensure enough shares are provided
        println!("Threshold  {:?}", threshold);
        println!("Share count  {:?}", share_count);
        if share_count < threshold {
            return Err(SeedMixerError::NotEnoughThresholdSharesProvided);
        }

        if share_count > threshold {
            return Err(SeedMixerError::TooManyThresholdSharesProvided);
        }

        let mut grouped: HashMap<usize, Vec<(BigInt, BigInt)>> =
            HashMap::with_capacity(seed_length);

        // Group shares by their index
        for share in shares.clone().values() {
            for (index, value) in share.shares.iter().enumerate() {
                grouped
                    .entry(index)
                    .or_default()
                    .push((share.index.clone(), value.clone()));
            }
        }

        // Reconstruct the secrets
        let shamir = Shamir::new(
            total.to_usize().unwrap_or_default(),
            threshold.to_usize().unwrap_or_default(),
        );
        let mut indices: HashMap<usize, BigInt> = HashMap::with_capacity(seed_length);
        for collected in grouped.into_iter() {
            let value = collected.1.as_slice();
            let reconstructed = shamir.reconstruct_secret(value)?;
            indices.insert(collected.0, reconstructed);
        }

        // Sort indices by key and convert to `u16`
        let mut sorted_indices: Vec<_> = indices.clone().into_iter().collect();
        sorted_indices.sort_by_key(|&(key, _)| key);
        let indices: Vec<u16> = sorted_indices
            .clone()
            .into_iter()
            .map(|a| a.1.to_u16().unwrap_or_default())
            .collect();

        // Deobfuscate the seed
        let obfuscated_seed = ObfuscatedSeed {
            indices,
            length: seed_length as u16,
        };

        let unshifter = PowerOfTwoUnshift {};
        let seed = obfuscated_seed.deobfuscate(pin, unshifter)?;

        Ok(seed)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use proptest::prelude::*;

    fn indices_strategy() -> impl Strategy<Value = Vec<u16>> {
        // Generate a HashSet of exactly 24 distinct u16s, each in [1..=2048].
        proptest::collection::vec(1u16..=2048, 1..=24)
            .prop_filter("Must have lengths 12,16 or 24", |s| {
                s.len() == 12 || s.len() == 16 || s.len() == 24
            })
    }

    fn pin_strategy() -> impl Strategy<Value = Pin> {
        proptest::collection::vec(1u16..10, 4).prop_map(Pin)
    }

    #[test]
    fn test_mix_unmix_cycle() {
        proptest!(|(unique_vec in indices_strategy(), pin in pin_strategy())| {
            let seed =Seed::new(unique_vec).unwrap();
                    let total_shares = 5;
                    let threshold = 3;

                    let indices = seed.indices.clone();

        println!("Indices {:?}",&indices);
        println!("Indices Length {:?}",&indices.len());


                    // Test mix
                    let shares:HashMap<BigInt,SecretShare> = SeedMixer::mix(seed, &pin, total_shares, threshold)
                        .expect("Mixing failed");

                    let recovery_shares:HashMap<BigInt,SecretShare> = shares.into_iter().take(threshold).collect();

                    // Test unmix
                    let reconstructed_seed = SeedMixer::unmix(recovery_shares, &pin)
                        .expect("Unmixing failed");

                    prop_assert_eq!(indices, reconstructed_seed.indices);
                });
    }
}
