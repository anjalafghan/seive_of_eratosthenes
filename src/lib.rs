// src/lib.rs
pub fn find_primes_dumb(input_vector: &mut Vec<i32>) {
    input_vector.retain(|&x| x % 2 != 0 || x == 2);
    input_vector.retain(|&x| x % 3 != 0 || x == 3);
    input_vector.retain(|&x| x % 5 != 0 || x == 5);
    input_vector.retain(|&x| x % 7 != 0 || x == 7);
}

pub fn find_primes_smart(input_vector: &mut Vec<i32>) {
    input_vector.retain(|&x| is_a_prime(x));
}

pub fn is_a_prime(n: i32) -> bool {
    if n == 2 || n == 3 || n == 5 || n == 7 {
        return true;
    }
    let limit = (n as f64).sqrt() as i32 + 1;
    for i in (11..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
