use chiphers::cipher::{rsa, Cipher};

use std::io;

fn main() {
    println!("Input the string you want to encrypt:");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    let encrypted_input = rsa::Rsa::new(user_input).expect("");
    let encrypted_string = encrypted_input.encrypted_string().expect("");

    println!("Your encrypted string: \r\n {}", encrypted_string);
    println!("Your original string: \r\n {}", encrypted_input.original_string().expect(""));
}
