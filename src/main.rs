fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    #[test]
    pub fn test_that_passes() {}

    #[test]
    pub fn test_that_fails() {
        assert_eq!(1, 0)
    }

    #[test]
    pub fn test_is_flakey_fails_retry_flakey() {
        if thread_rng().gen_bool(0.5) {
            assert_eq!(1, 0)
        } else {
            assert_eq!(0, 0)
        }
    }
}
