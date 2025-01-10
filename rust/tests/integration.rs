use num_traits::cast::FromPrimitive;
use seedmixer::{
    collection::{seed::SeedCollector, share::ShareCollector},
    mix::{SecretShare, SeedMixer, SeedMixing},
    obfuscation::Pin, // Adjust the path as needed
    seed::Seed,
    words::WordList,
};

use num_bigint::BigInt;
use proptest::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

fn arb_language() -> impl Strategy<Value = &'static str> {
    prop_oneof![Just("en"), Just("es"), Just("fr"), Just("de"),]
}

fn arb_n_of_m() -> impl Strategy<Value = (usize, usize)> {
    prop_oneof![
        Just((3usize, 5usize)),
        Just((2usize, 3usize)),
        Just((4usize, 5usize)),
    ]
}
/*
fn arb_pin() -> impl Strategy<Value = Pin> {
    prop::collection::vec(1..10u16, 4).prop_map(|v| Pin(v))
}*/
fn arb_pin() -> impl Strategy<Value = Pin> {
    prop::sample::select(vec![4, 6, 12, 24]) // Select a size
        .prop_flat_map(|size| {
            prop::collection::vec(1..10u16, size) // Generate a vector with the selected size
        })
        .prop_map(|v| Pin(v)) // Map the vector into a `Pin`
}

fn pick_n_words(word_list: &WordList, n: usize) -> Result<Vec<String>, &'static str> {
    let all_words: Vec<String> = word_list.words.values().cloned().collect(); // Adjust this if your API differs
    if all_words.len() < n {
        return Err("Not enough words in the word list to pick 24 unique words");
    }

    let mut rng = thread_rng();
    let chosen_words: Vec<String> = all_words
        .clone()
        .as_slice()
        .choose_multiple(&mut rng, n)
        .cloned()
        .collect();

    Ok(chosen_words)
}
fn select_n_of_m(n: usize, m: usize) -> Vec<usize> {
    // Create a vector of integers from 0 to m-1
    let all: Vec<usize> = (1..=m).collect();

    let mut rng = thread_rng();
    let chosen: Vec<usize> = all
        .as_slice()
        .choose_multiple(&mut rng, n) // select n distinct items
        .cloned() // convert &usize to usize
        .collect();

    chosen
}

struct TestSeedCollector {
    wl: WordList,
    words: Vec<String>,
}

impl TestSeedCollector {
    fn new(wl: WordList, words: Vec<String>) -> Self {
        Self { wl, words }
    }
}

impl SeedCollector for TestSeedCollector {
    fn collect(&self) -> Result<seedmixer::seed::Seed, seedmixer::errors::CollectionError> {
        let indices: Vec<u16> = self
            .words
            .clone()
            .into_iter()
            .map(|w| self.wl.get_index(&w))
            .map(|i| *i)
            .collect();

        let length = indices.clone().len() as u16;
        Ok(Seed { indices, length })
    }
}

struct TestShareCollector {
    shares: HashMap<BigInt, SecretShare>,
}

impl TestShareCollector {
    fn new(shares: HashMap<BigInt, SecretShare>) -> Self {
        Self { shares }
    }
}

impl ShareCollector for TestShareCollector {
    fn collect(&self) -> Result<HashMap<BigInt, SecretShare>, seedmixer::errors::CollectionError> {
        Ok(self.shares.clone())
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    #[test]
    fn test_mix_unmix_n_of_m_unencrypted(lang in arb_language(), pin in arb_pin(), (n,m) in arb_n_of_m() ) {
        let word_list = WordList::new(&lang.to_string());
        let seed_words = pick_n_words(&word_list,24usize).expect("Could not generate test seed");
        let test_seed_collector= TestSeedCollector::new(word_list,seed_words);
        let seed = test_seed_collector.collect().unwrap();

        let result = SeedMixer::mix(
            seed,
            &pin,
            m,
            n,
        );

        prop_assert!(result.is_ok(), "mix failed with error: {:?}", result.err());

        let shares=result.unwrap();

        let mut shares_selection:HashMap<BigInt,SecretShare>=HashMap::new();

         for i in select_n_of_m(n,m){
            let bi = BigInt::from_usize(i).unwrap();
            let share:SecretShare=shares.get(&bi).unwrap().clone();
            shares_selection.insert(bi,share);
         }


         let test_share_collector=TestShareCollector::new(shares_selection);
         let shares_to_unmix:HashMap<BigInt,SecretShare>=test_share_collector.collect().unwrap();


        // Unmix
        let retrieve_result = SeedMixer::unmix(
            shares_to_unmix,
            &pin
        );

        prop_assert!(retrieve_result.is_ok(), "unmix failed with error: {:?}", retrieve_result.err());

        let expected_seed = test_seed_collector.collect().unwrap();
        let result_seed= retrieve_result.unwrap();
        assert_eq!(&expected_seed.length,&result_seed.length);
        assert_eq!(&expected_seed.indices,&result_seed.indices);

    }
}
