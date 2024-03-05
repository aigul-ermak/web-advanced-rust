use std::error::Error;

pub mod rsa;

pub trait Cipher {
    fn original_string(&self) -> Result<String, Box<dyn Error>>;
    fn encrypted_string(&self) -> Result<String, Box<dyn Error>>;    
}