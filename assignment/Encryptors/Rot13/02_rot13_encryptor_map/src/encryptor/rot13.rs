pub struct Rot13(pub String);

impl super::Encryptable for Rot13 {
    fn encrypt(&self) -> String {
        self.0
            .chars()
            .map(|ch| {
                //TODO - Implement Rot13 encryption
            })
            .collect()
    }
}
