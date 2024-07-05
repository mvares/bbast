#[derive(Debug)]
pub enum TokenType {
    Literal,
    Operator,
}

#[derive(Debug)]
pub struct Token {
    #[allow(dead_code)]
    pub value: String,
    #[allow(dead_code)]
    pub r#type: TokenType,
}

impl TokenType {
    fn is_int(expr: &str) -> bool {
        expr.chars().all(|c| matches!(c, '0'..='9'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_passes() {
        assert!(TokenType::is_int("1234"));
        assert!(TokenType::is_int("0234"));
    }
}
