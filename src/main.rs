use rand::Rng;
use std::cmp::Ordering; // Ordering is an enum
use std::io;

fn game() {
    println!("Guess the number!");

    // variabile statica (costante)
    let secret_number = rand::thread_rng().gen_range(1..101);

    // il punto esclamativo indica che è una macro (la macro println! è simile a quella di C)
    println!("The secret number is: {}", secret_number);

    loop {
        // variabile mutabile
        let mut guess = String::new();

        println!("Please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess
            .trim() // remove whitespace
            .parse() // parse string to number
            .expect("Please type a number!"); // handle error

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                // break the loop
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {
    let mut config = String::new();

    println!("Hello, world!");
    println!("A che gioco vuoi giocare?");
    println!("1. Guessing game");

    io::stdin()
        .read_line(&mut config)
        .expect("Failed to read line");

    let config: u32 = config
        .trim() // remove whitespace
        .parse() // parse string to number
        .expect("Please type a number!"); // handle error

    if config == 1 {
        game();
    } else {
        println!("I don't know this game!");
    }
}
