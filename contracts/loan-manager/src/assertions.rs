pub fn require(invariant: bool, error: &str) {
    if !invariant {
        panic!("{}", error);
    }
}