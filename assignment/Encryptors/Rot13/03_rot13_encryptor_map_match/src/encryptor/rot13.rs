pub struct Rot13(pub String);

impl super::Encryptable for Rot13 {
    fn encrypt(&self) -> String {
        self.0
            .chars()
            .map(|ch| match ch {
                //TODO - Implement the ROT13 algorithm
            })
            .collect()
    }
}