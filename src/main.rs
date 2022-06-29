use rand::prelude::*;
use std::env;

fn main() {
    let mut rng = thread_rng();
    let mut co_primes: usize = 0;
    let limit = env::args().nth(1).unwrap().parse::<usize>().unwrap();

    for _i in 0..limit {
        let a: u64 = rng.gen();
        let b: u64 = rng.gen();

        if gcd(a, b) == 1 {
            co_primes += 1
        }
    }

    let ratio = co_primes as f64 / limit as f64;
    println!("pi: {}", (6.0 / ratio).sqrt());
}

fn gcd(a: u64, b: u64) -> u64 {
    if b != 0 {
        return gcd(b, a % b);
    }

    a
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 7 * 11 * 13, 3 * 5 * 7 * 11 * 13), 3 * 5 * 7 * 11 * 13);
}