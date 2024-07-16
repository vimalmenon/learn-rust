#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let number: u8 = 5;
        assert_eq!(add(&number), 10);
    }

    #[test]
    fn test_add_with_zero() {
        let number: u8 = 0;
        assert_eq!(add(&number), 5);
    }

    #[test]
    fn test_add_with_max_value() {
        let number: u8 = u8::MAX - 5;
        assert_eq!(add(&number), u8::MAX);
    }

    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn test_add_overflow() {
        let number: u8 = u8::MAX;
        add(&number);
    }
}
