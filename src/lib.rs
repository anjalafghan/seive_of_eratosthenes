use std::collections::HashSet;

pub fn find_primes(input_vector: &mut Vec<i32>, primes: &HashSet<i32>) {
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
