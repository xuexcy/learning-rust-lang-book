use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub mod guess;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read_line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The secret_number will be between 1 and 100.");
            continue;
        }
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    secret_number_func();

}

fn secret_number_func() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess.");
        let mut my_guess = String::new();
        io::stdin().read_line(&mut my_guess).expect("Failed to read_line");
        let my_guess = match my_guess.trim().parse() {
            Ok(num) => guess::Guess::new(num).value(),
            Err(_) => continue,
        };
        match my_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
