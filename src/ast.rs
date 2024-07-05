use crate::token::{Token, TokenType};

#[derive(Debug)]
#[allow(dead_code)]
pub struct BinaryAST(Vec<Token>);

impl BinaryAST {
    #[allow(dead_code)]
    pub fn new(input: &str) -> Self {
        let mut tokens = Vec::new();
        let mut buffer = String::new();
        let mut iter = input.chars().peekable();

        while let Some(&c) = iter.peek() {
            if !c.is_whitespace() && c.is_digit(10) {
                while let Some(&c) = iter.peek() {
                    if c.is_digit(10) {
                        buffer.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }

                tokens.push(Token {
                    value: buffer.clone(),
                    r#type: TokenType::Literal,
                });

                buffer.clear();
            } else if matches!(c, '+' | '-' | '*' | '/') {
                tokens.push(Token {
                    value: c.to_string(),
                    r#type: TokenType::Operator,
                });

                iter.next();
            } else {
                iter.next();
            }
        }

        BinaryAST(tokens)
    }
}
