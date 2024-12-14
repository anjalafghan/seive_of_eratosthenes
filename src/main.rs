use seive_of_eratosthenes::{find_primes_dumb, find_primes_smart};

fn main() {
    let input = get_valid_input();
    let mut input_vector: Vec<i32> = (2..=input).collect();
    find_primes_dumb(&mut input_vector);
    println!("Dumb primes: {:?}", input_vector);
    find_primes_smart(&mut input_vector);
    println!("Smart primes: {:?}", input_vector);
}

fn get_valid_input() -> i32 {
    use std::io;

    loop {
        println!("Please input a number: ");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<i32>() {
                Ok(number) => return number,
                Err(_) => println!("Invalid input, please try again."),
            },
            Err(e) => println!("Error reading input: {e}"),
        }
    }
}
