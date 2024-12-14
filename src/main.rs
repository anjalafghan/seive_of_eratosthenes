use std::io;



fn main() {
    let input = get_valid_input();
    let input_vector: Vec<i32> = (2..=input).collect();

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


