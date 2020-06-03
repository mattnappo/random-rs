use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Guessing Game");

    let secret = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Guess: ");

        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not a number!");
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
