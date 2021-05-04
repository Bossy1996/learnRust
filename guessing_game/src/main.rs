use std::io; // input/output standar library
use rand::Rng; // random number generator crate
use std::cmp::Ordering; // Stuff

fn main() {
    println!("Guess the number!"); // prints the string to the screen

    let secret_number = rand::thread_rng().gen_range(1, 101); // it generates a randon number between 1 and 100

    println!("The secret number is {}", secret_number);

    loop {

        println!("Please input your guess."); // prints the string to the screen

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
