#[cfg(test)]
mod test {
    use testing::*;

    #[test]
    fn sploosh_integration() {
        assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4)
    }
}
