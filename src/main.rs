use std::io;



fn main() {
    let input = get_valid_input();
    let mut input_vector: Vec<i32> = (2..=input).collect();
    input_vector.retain(|&x| x % 2 != 0 || x == 2);
    input_vector.retain(|&x| x % 3 != 0 || x == 3);
    input_vector.retain(|&x| x % 5 != 0 || x == 5);
    input_vector.retain(|&x| x % 7 != 0 || x == 7);
    println!("The list of all prime numbers are {:?}" , input_vector)


}

fn get_valid_input() -> i32{
    loop {
        println!("Please input a number \n");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(number) => {
                        println!("The number entered {number} is a valid number");
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


