pub struct Rot13(pub String);

impl super::Encryptable for Rot13 {
    fn encrypt(&self) -> String {
        let mut new_string = String::new();
        for ch in self.0.chars() {
            //TODO - Implement the ROT13 algorithm
        }
        new_string
    }
}
