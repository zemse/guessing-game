use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("This is a game of guessing!");

    loop {
        println!("\nPlease input your guess:");

        let mut guess = String::new();

        stdin().read_line(&mut guess)
            .expect("Failed to load line");

        /// @dev trim method on string eleminates white spaces begnining and end
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        println!("Your guess is {}", guess);

        let secret_number = rand::thread_rng().gen_range(1,5);
        // println!("secret_number: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}
