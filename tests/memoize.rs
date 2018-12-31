use memoize_macro::memoize;
use std::collections::{BTreeMap, HashMap};

#[memoize] // uses hashmap by default
fn fib_default(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fib_default(n - 2) + fib_default(n - 1),
    }
}

#[memoize(HashMap)]
fn fib_hash(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fib_hash(n - 2) + fib_hash(n - 1),
    }
}

#[memoize(BTreeMap)]
fn fib_btree(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fib_btree(n - 2) + fib_btree(n - 1),
    }
}

#[test]
fn it_works() {
    let output: Vec<_> = (0..50).map(fib_default).collect();
    assert_eq!(
        output,
        vec![
            1,
            1,
            2,
            3,
            5,
            8,
            13,
            21,
            34,
            55,
            89,
            144,
            233,
            377,
            610,
            987,
            1597,
            2584,
            4181,
            6765,
            10946,
            17711,
            28657,
            46368,
            75025,
            121_393,
            196_418,
            317_811,
            514_229,
            832_040,
            1_346_269,
            2_178_309,
            3_524_578,
            5_702_887,
            9_227_465,
            14_930_352,
            24_157_817,
            39_088_169,
            63_245_986,
            102_334_155,
            165_580_141,
            267_914_296,
            433_494_437,
            701_408_733,
            1_134_903_170,
            1_836_311_903,
            2_971_215_073,
            4_807_526_976,
            7_778_742_049,
            12_586_269_025,
        ]
    );
}
