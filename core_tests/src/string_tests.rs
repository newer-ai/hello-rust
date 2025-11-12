#[cfg(test)]
mod tests {

    #[test]
    fn test_str_len() {
        let str = "你好";

        assert_eq!(str.len(), 6);
        assert_eq!(str.chars().count(), 2);
    }
}
