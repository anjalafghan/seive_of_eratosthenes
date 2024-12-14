use std::io;


fn main() {
    let input = get_valid_input();
    let mut input_vector: Vec<i32> = (2..=input).collect();
    find_primes_dumb(&mut input_vector);
    find_primes_smart(&mut input_vector);
}

fn find_primes_smart(input_vector: &mut Vec<i32>) {
    input_vector.retain(|&x| is_a_prime(x));
    println!("The list of all prime numbers are {:?}", input_vector)
}

fn is_a_prime(n: i32) -> bool {
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

fn find_primes_dumb(input_vector: &mut Vec<i32>) {
    input_vector.retain(|&x| x % 2 != 0 || x == 2);
    input_vector.retain(|&x| x % 3 != 0 || x == 3);
    input_vector.retain(|&x| x % 5 != 0 || x == 5);
    input_vector.retain(|&x| x % 7 != 0 || x == 7);
    println!("The list of all prime numbers are {:?}", input_vector)
}

fn get_valid_input() -> i32 {
    loop {
        println!("Please input a number \n");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(number) => {
                        return number;
                    }
                    Err(_) => {
                        println!("Unfortunately the number you entered is not a valid number");
                    }
                }
            }
            Err(e) => {
                println!("Error reading the valid input {e}")
            }
        }
    }
}


