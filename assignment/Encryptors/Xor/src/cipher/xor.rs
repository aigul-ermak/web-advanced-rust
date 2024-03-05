use std::error::Error;

pub struct Xor(
    pub String,
    pub char
);

impl super::Cipher for Xor {
    fn original_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from(&self.0))
    }
    fn encrypted_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(self.0
            .chars()
            .map(|ch|( /*TODO - Implement XOR encrypter here */)
            .collect())
    }
}