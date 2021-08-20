use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");
        // variables are immutable by default, add mut to make them mutable
        let mut guess = String::new();
        // read_line returns a Result, which must be handled because it can be of type Err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // crash on Err
        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // no new scope created for match block
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
