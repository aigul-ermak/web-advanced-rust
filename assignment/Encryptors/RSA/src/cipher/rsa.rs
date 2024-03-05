use std::error::Error;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};


pub struct Rsa{
    data: String,
    private_key: RsaPrivateKey,
}

const KEY_SIZE : usize = 2048;


impl Rsa {
    pub fn new (input: String) -> Result<Self, Box<dyn Error>> {
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, KEY_SIZE).expect("Failed to generate a key");
        let public_key = RsaPublicKey::from(&private_key);
        let input_bytes = input.as_bytes();
        let encrypted_data = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, input_bytes).expect("Failed to encrypt");
        let encoded_data = STANDARD.encode(encrypted_data);
        Ok(Self { data: encoded_data, private_key: private_key })
    }
}

impl super::Cipher for Rsa {
    fn original_string(&self) -> Result<String, Box<dyn Error>> {
        let decoded_data = STANDARD.decode(&self.data).expect("Fail to decode the data");
        let descrypted_data = self
            .private_key
            .decrypt(Pkcs1v15Encrypt, &decoded_data).expect("Failed to decrypt");
        Ok(String::from_utf8(descrypted_data).expect("Failed convertion to UTF-8"))

    }
    fn encrypted_string(&self) -> Result<String, Box<dyn Error>> {
        Ok(String::from(&self.data))
    }
}