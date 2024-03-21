use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let name = name.trim();
    println!("Hello, {}! Nice to meet you.", name);

    println!("How old are you?");
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Failed to read line");
    
    let age: u32 = age.trim().parse().expect("Please type a numer!");

    // Calculate the birth year
    let current_year = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() / 60 / 60 / 24 / 365 + 1970; // Convert the current time to years

    let birth_year = current_year - age as u64;
    println!("{}! Based on your age, you were born in or around {}.", name, birth_year);
}
