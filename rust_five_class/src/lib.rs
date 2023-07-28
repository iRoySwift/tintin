#[macro_export]
macro_rules! string {
    ($a:expr) => {{
        let string = String::from($a);
        string
    }};
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string() {
        assert_eq!(string!("Hello Rust!"), String::from("Hello Rust!"));
    }
}
