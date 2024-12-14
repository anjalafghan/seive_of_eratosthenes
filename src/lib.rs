use std::collections::HashSet;

fn sieve_of_eratosthenes(limit: i32) -> HashSet<i32> {
    let mut sieve = vec![true; (limit + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=(limit as f64).sqrt() as i32 {
        if sieve[i as usize] {
            for j in (i * i..=limit).step_by(i as usize) {
                sieve[j as usize] = false;
            }
        }
    }
    sieve.into_iter()
        .enumerate()
        .filter_map(|(i, is_prime)| if is_prime { Some(i as i32) } else { None })
        .collect()
}

pub fn find_primes_dumb(input_vector: &mut Vec<i32>, primes: &HashSet<i32>) {
    input_vector.retain(|&x| primes.contains(&x) || is_a_prime(x, primes));
}

pub fn find_primes_smart(input_vector: &mut Vec<i32>, primes: &HashSet<i32>) {
    input_vector.retain(|&x| primes.contains(&x) || is_a_prime(x, primes));
}

fn is_a_prime(n: i32, primes: &HashSet<i32>) -> bool {
    if n <= 1 {
        return false;
    }
    if primes.contains(&n) {
        return true;
    }
    let limit = (n as f64).sqrt() as i32;
    for &prime in primes.iter().filter(|&&prime| prime <= limit) {
        if n % prime == 0 {
            return false;
        }
    }
    true
}
