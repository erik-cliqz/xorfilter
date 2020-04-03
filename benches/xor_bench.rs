#![feature(test)]
extern crate test;

use rand::{prelude::random, rngs::SmallRng, Rng, SeedableRng};
use test::Bencher;
use xorfilter::Xor8;

#[bench]
fn bench_populate_keys_100000(b: &mut Bencher) {
    let seed: u128 = random();
    let mut rng = SmallRng::from_seed(seed.to_le_bytes());

    let testsize = 100_000;
    let mut keys: Vec<u64> = Vec::with_capacity(testsize);
    keys.resize(testsize, Default::default());
    for key in keys.iter_mut() {
        *key = rng.gen();
    }

    b.iter(|| {
        let mut filter = Xor8::new();
        filter.populate_keys(&keys);
        filter.build();
    })
}

#[bench]
fn bench_build_keys_100000(b: &mut Bencher) {
    let seed: u128 = random();
    let mut rng = SmallRng::from_seed(seed.to_le_bytes());

    let testsize = 100_000;
    let mut keys: Vec<u64> = Vec::with_capacity(testsize);
    keys.resize(testsize, Default::default());
    for key in keys.iter_mut() {
        *key = rng.gen();
    }

    b.iter(|| {
        let mut filter = Xor8::new();
        filter.build_keys(&keys);
    })
}

#[bench]
fn bench_populate_100000(b: &mut Bencher) {
    let seed: u128 = random();
    let mut rng = SmallRng::from_seed(seed.to_le_bytes());

    let testsize = 100_000;
    let mut keys: Vec<u64> = Vec::with_capacity(testsize);
    keys.resize(testsize, Default::default());
    for key in keys.iter_mut() {
        *key = rng.gen();
    }

    b.iter(|| {
        let mut filter = Xor8::new();
        filter.populate(&keys);
        filter.build();
    })
}

#[bench]
fn bench_insert_100000(b: &mut Bencher) {
    let seed: u128 = random();
    let mut rng = SmallRng::from_seed(seed.to_le_bytes());

    let testsize = 100_000;
    let mut keys: Vec<u64> = Vec::with_capacity(testsize);
    keys.resize(testsize, Default::default());
    for key in keys.iter_mut() {
        *key = rng.gen();
    }

    b.iter(|| {
        let mut filter = Xor8::new();
        keys.iter().for_each(|key| filter.insert(key));
        filter.build();
    })
}

#[bench]
fn bench_contains_100000(b: &mut Bencher) {
    let seed: u128 = random();
    let mut rng = SmallRng::from_seed(seed.to_le_bytes());

    let testsize = 100_000;
    let mut keys: Vec<u64> = Vec::with_capacity(testsize);
    keys.resize(testsize, Default::default());
    for key in keys.iter_mut() {
        *key = rng.gen();
    }

    let filter = {
        let mut filter = Xor8::new();
        filter.populate(&keys);
        filter.build();
        filter
    };

    let mut n = 0;
    b.iter(|| {
        filter.contains(keys[n % keys.len()]);
        n += 1;
    });
}

#[bench]
fn bench_contains_key_100000(b: &mut Bencher) {
    let seed: u128 = random();
    let mut rng = SmallRng::from_seed(seed.to_le_bytes());

    let testsize = 100_000;
    let mut keys: Vec<u64> = Vec::with_capacity(testsize);
    keys.resize(testsize, Default::default());
    for key in keys.iter_mut() {
        *key = rng.gen();
    }

    let filter = {
        let mut filter = Xor8::new();
        filter.populate(&keys);
        filter.build();
        filter
    };

    let mut n = 0;
    b.iter(|| {
        filter.contains_key(keys[n % keys.len()]);
        n += 1;
    });
}
