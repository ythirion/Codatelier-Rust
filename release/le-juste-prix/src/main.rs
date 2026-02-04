#![allow(dead_code, unused_imports)]

use std::cmp::Ordering;
use std::io;

// release
fn main() {
    println!("== Le juste prix ==");
    let juste_prix = generate_random_number_between(1, 100);
    let mut found = false;

    while !found {
        println!("Quel est le juste prix ?");

        match get_input_from_user().trim().parse::<u32>() {
            Ok(guess) => found = process_number(guess, juste_prix),
            Err(e) => println!("Erreur: {}", e),
        }
    }
}

fn process_number(guess: u32, juste_prix: u32) -> bool {
    match guess {
        number if number == juste_prix => {
            println!("Gagné !!! 🍌🍌🍌");
            true
        }
        number if number < juste_prix => {
            println!("{} est trop petit", guess);
            false
        }
        _ => {
            println!("{} est trop grand", guess);
            false
        }
    }
}

/// Generates a random u32 number between min and max included.
fn generate_random_number_between(min: u32, max: u32) -> u32 {
    use rand::Rng;

    rand::rng().random_range(min..=max)
}

/// lit l'entrée standard
fn get_input_from_user() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur: ligne non lue.");

    input
}
