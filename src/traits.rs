pub trait Hashable {
    fn hashable_data(&self) -> Vec<u8>;
    fn set_hash(&mut self);

    fn hash_data(&self) -> String {
        self.hash(&self.hashable_data())
    }

    fn hash(&self, input: &[u8]) -> String {
        blake3::hash(input).to_string()
    }
}
