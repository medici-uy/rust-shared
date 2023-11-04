pub trait Hashable {
    fn hashable_data(&self) -> Vec<u8>;

    fn hash(&self) -> String {
        blake3::hash(&self.hashable_data()).to_string()
    }
}
