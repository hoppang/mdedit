#[cfg(test)]
mod cjkstring_tests {
    use crate::cjkstring::CjkString;

    #[test]
    fn test_from() {
        let s: CjkString = CjkString::from("안녕");
        assert_eq!(s.s, "안녕");
        assert_eq!(s.idx, 0);
    }
}
