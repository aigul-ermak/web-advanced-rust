use chiphers::cipher::{xor, Cipher};

use std::io;

fn main() {
    println!("Input the string you want to encrypt:");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    let char_seed = 'm';
    let encrypted_message = xor::Xor(user_input, char_seed).encrypted_string().expect("");
    println!("Your encrypted by Xor string is: \r\n {}", encrypted_message);
    println!("Your original word by Xor is: \r\n {}", xor::Xor(encrypted_message, char_seed).encrypted_string().expect(""));
}
