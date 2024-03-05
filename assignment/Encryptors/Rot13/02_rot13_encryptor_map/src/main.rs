// 4. define the new module encryptor
pub mod encryptor;

// 8. use the Encryptable trait and the Rot13 structure
use encryptor::Encryptable;

use std::io;
fn main() {
    println!("Input the string you want to encrypt:");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read input");

    let encrypted_message = encryptor::rot13::Rot13(user_input).encrypt();
    
    print!("Your encrytperd string is: {}", encrypted_message);
    print!("Your original string is: {}", encryptor::rot13::Rot13(encrypted_message).encrypt());
}
