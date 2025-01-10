use num_bigint::RandBigInt;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
use once_cell::sync::Lazy;

use crate::errors::ShamirError;

// A 1024-bit prime number defined with once_cell's Lazy for lazy initialization.
static PRIME: Lazy<Result<BigUint, ShamirError>> = Lazy::new(|| {
    "7876392013106067884694803200456018364629049395434127864807249940946137609495271847985527744803990426633202611333011018935827714873253604323773749390637467"
        .parse::<BigUint>()
        .map_err(ShamirError::UnableToParseBigInt)
});

fn get_prime() -> Result<&'static BigUint, ShamirError> {
    Lazy::force(&PRIME).as_ref().map_err(|s| s.clone())
}

fn get_prime_as_bigint() -> Result<BigInt, ShamirError> {
    match get_prime()?.to_bigint() {
        Some(v) => Ok(v),
        None => Err(ShamirError::UnableToConvertBigUintToBigInt),
    }
}

/// A struct to manage Shamir's Secret Sharing generation and reconstruction.
pub struct Shamir {
    /// The total number of shares to generate.
    total_shares: usize,
    /// The threshold number of shares needed to reconstruct the secret.
    threshold: usize,
}

impl Shamir {
    /// Creates a new `Shamir` struct with a given total number of shares and threshold.
    pub fn new(total_shares: usize, threshold: usize) -> Self {
        Self {
            total_shares,
            threshold,
        }
    }

    /// Generates random coefficients for a polynomial of degree `k - 1`.
    /// `a_0` is the secret, and the rest are random coefficients modulo PRIME.
    fn generate_coefficients(
        &self,
        secret: &BigUint,
        k: usize,
    ) -> Result<Vec<BigUint>, ShamirError> {
        let mut rng = rand::thread_rng();
        let mut coefficients = vec![secret.clone()];

        let p = get_prime()?;
        // Generate random coefficients for a polynomial of degree k - 1.
        for _ in 1..k {
            coefficients.push(rng.gen_biguint_below(p));
        }

        Ok(coefficients)
    }

    /// Evaluates the polynomial at a given x using Horner's method, modulo `PRIME`.
    fn evaluate_polynomial(coefficients: &[BigUint], x: &BigInt) -> Result<BigInt, ShamirError> {
        let p = get_prime_as_bigint()?;

        let mut result = BigInt::zero();

        // Horner's method: result = (...((0 * x + a_n) * x + a_{n-1}) ... ) mod p
        for coeff in coefficients.iter().rev() {
            let c = match (coeff).to_bigint() {
                Some(v) => Ok(v),
                None => Err(ShamirError::UnableToConvertBigUintToBigInt),
            }?;
            result = (result * x + c) % &p;
        }
        Ok(result)
    }

    // Generates n shares, where each share is a (x, y) point.
    pub fn generate_shares(&self, secret: &BigUint) -> Result<Vec<(BigInt, BigInt)>, ShamirError> {
        // Get polynomial coefficients: [a_0, a_1, ..., a_{k-1}]
        let coefficients = self.generate_coefficients(secret, self.threshold)?;
        let mut shares = Vec::new();
        for i in 1..=self.total_shares {
            let x = i.to_bigint().unwrap();
            let y = Self::evaluate_polynomial(&coefficients, &x)?;
            shares.push((x, y));
        }

        Ok(shares)
    }

    /// Reconstructs the secret using Lagrange interpolation at x = 0. Expects at least `threshold` shares.
    pub fn reconstruct_secret(&self, shares: &[(BigInt, BigInt)]) -> Result<BigInt, ShamirError> {
        let mut secret = BigInt::zero();
        let p = get_prime_as_bigint()?;

        // For each share, build the Lagrange basis polynomial and sum up.
        for (i, (xi, yi)) in shares.iter().enumerate() {
            let mut num = BigInt::one();
            let mut denom = BigInt::one();

            // Construct numerator and denominator for Lagrange basis polynomial.
            for (j, (xj, _)) in shares.iter().enumerate() {
                if i != j {
                    num = (num * xj) % &p;
                    denom = (denom * (xj - xi)) % &p;
                }
            }

            // Compute the modular inverse using Fermat's Little Theorem (p is prime).
            // denom^(-1) mod p = denom^(p-2) mod p
            let denom_inv = denom.modpow(&(&p - 2_u32), &p); // Fermat's Little Theorem for modular inverse
            let term = (yi * num * denom_inv) % &p;
            secret = (secret + term) % &p;
        }

        // Ensure the secret is in the range [0, p-1].
        Ok((secret + &p) % &p)
    }
}

#[cfg(test)]
mod tests {
    use proptest::{prop_assert_eq, prop_oneof, strategy::Just};
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use {
        super::*,
        proptest::{
            prelude::{ProptestConfig, Strategy},
            proptest,
        },
    };

    fn biguint_strategy(min: u32, max: u32) -> impl Strategy<Value = BigUint> {
        (min..=max)
            .prop_map(|x| BigUint::from(x))
            .prop_filter("Value must be non-zero", |x| *x > BigUint::from(0u32))
    }

    fn n_of_m_strategy() -> impl Strategy<Value = (usize, usize)> {
        prop_oneof![
            Just((3usize, 5usize)),
            Just((2usize, 3usize)),
            Just((4usize, 5usize)),
        ]
    }

    #[test]
    fn test_generate_shares_basic() {
        // Suppose we have a small secret (just for demonstration).
        let secret = BigUint::from(1234u32);
        let shamir = Shamir::new(5, 3);
        let shares = shamir.generate_shares(&secret).unwrap();

        // We expect 5 shares
        assert_eq!(shares.len(), 5);

        // We can try reconstructing from exactly 3 shares
        let subset = &shares[0..3];
        let reconstructed = shamir.reconstruct_secret(subset).unwrap();
        assert_eq!(reconstructed, secret.to_bigint().unwrap());
    }

    #[test]
    fn test_reconstruct_with_all_shares() {
        let secret = BigUint::from(999u32);
        let shamir = Shamir::new(5, 3);
        let shares = shamir.generate_shares(&secret).unwrap();

        // Reconstruct from all 5 shares, which should also work fine
        let reconstructed = shamir.reconstruct_secret(&shares).unwrap();
        assert_eq!(reconstructed, secret.to_bigint().unwrap());
    }

    proptest! {
           #![proptest_config(ProptestConfig::with_cases(1000))]
           #[test]
           fn test_share_and_reconstruct(secret in biguint_strategy(0,2048), (threshold,shares) in n_of_m_strategy() ){
           println!("{}",secret);
           let shamir=Shamir::new(shares,threshold);
           let shares=shamir.generate_shares(&secret).unwrap();

           let mut rng = thread_rng();
           let mut selected_values = shares.clone();
           selected_values.shuffle(&mut rng);

           let recovery_three: Vec<(BigInt,BigInt)> = selected_values.into_iter().take(threshold).collect();

           let recovered_secret= shamir.reconstruct_secret(&recovery_three.to_vec()).unwrap();
               prop_assert_eq!(&secret.to_bigint().unwrap(), &recovered_secret, "Expected the secret to be recoverable: {} vs recovered {}.  Shares were {:?}. Selected were {:?}", &secret,&recovered_secret,shares.clone(),recovery_three.clone());

        }
    }
}
