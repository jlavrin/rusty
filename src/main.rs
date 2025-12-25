use std::io;

use rand::Rng;
use std::cmp::Ordering;

fn handle_input(arg: &mut u32) {
    let mut guess = String::new();
    println!("Please input a number");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read a line");
    *arg = guess.trim().parse().unwrap_or_else(|_| 0);
}

fn main() {
    let mut random = rand::rng().random_range(1..=100);
    loop {
        let mut guess: u32 = 0;
        handle_input(&mut guess);

        match random.cmp(&guess) {
            Ordering::Greater => println!("To low!"),
            Ordering::Equal => {
                println!("Congrats!");
                break;
            }
            Ordering::Less => println!("To high!"),
        }

        println!("You guessed: {}", guess)
    }
}
