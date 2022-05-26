use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number.");
    println!("Input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("The secret number is: {}", secret_number);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        // we shadow our original guess
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too low"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;
            }
            Ordering::Greater => println!("too high"),
        }
    }
}
