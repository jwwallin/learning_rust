extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut str_guess = String::new();
    while str_guess.trim() != "quit" {

        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("Please input your guess");

        str_guess.clear();
        io::stdin().read_line(&mut str_guess)
            .expect("Failed to read line!");
        println!("{}", str_guess);

        let u_guess: u32 = match str_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match u_guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => println!("You win!")
        }

    }
}
