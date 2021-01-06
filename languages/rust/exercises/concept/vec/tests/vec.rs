#[cfg(test)]
mod tests {
    use vec::{grab, humans_and_computers, languages};
    #[test]
    #[ignore]
    fn it_has_languages() {
        assert_eq!(languages().len(), 5);
    }
    #[test]
    #[ignore]
    fn gets_fist() {
        assert_eq!(grab(0), Some("Rust".to_string()));
    }
    #[test]
    #[ignore]
    fn gets_last() {
        assert_eq!(grab(4), Some("Scheme".to_string()));
    }
    #[test]
    #[ignore]
    fn adds_human_languages() {
        let both = humans_and_computers(["官话".to_string(), "كِسوَهِل".to_string()].to_vec());
        assert_eq!(both[0], "Rust");
        assert_eq!(both[5], "官话");
        assert_eq!(both[6], "كِسوَهِل");
    }
}