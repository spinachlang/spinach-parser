#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let input = r#"
            q1: q2
            my_list: [1, 2, 3]
            inst: X | Y | Z
            q1 -> X(1)
        "#;
        
        let parser = Parser::new();
        let result = parser.get_tree(input);
        assert!(result.is_ok());
    }
}
