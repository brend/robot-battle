//! Tokenizer for the robot-battle DSL.
//!
//! This module provides functionality to tokenize lines and scripts written in the robot DSL.
//! The DSL supports commands such as `rotate`, `move`, `scan`, `fire`, and control flow like `if`, `else`, `while`.
//!
//! # Example
//!
//! ```
//! use tokenizer::{tokenize_line, Token};
//! let tokens = tokenize_line("rotate treads 90");
//! assert_eq!(
//!     tokens,
//!     vec![
//!         Token::Keyword("rotate".to_string()),
//!         Token::Identifier("treads".to_string()),
//!         Token::Number(90)
//!     ]
//! );
//! ```

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Number(i32),
    Symbol(char),
}

/// Tokenizes a single line of robot DSL code.
pub fn tokenize_line(line: &str) -> Vec<Token> {
    let mut words = line.split_whitespace();
    let mut tokens = Vec::new();
    if let Some(first) = words.next() {
        if [
            "rotate", "move", "scan", "fire", "if", "else", "while", "loop",
        ]
        .contains(&first)
        {
            tokens.push(Token::Keyword(first.to_string()));
        } else if let Ok(num) = first.parse::<i32>() {
            tokens.push(Token::Number(num));
        } else if first.len() == 1 && "{}()".contains(first) {
            tokens.push(Token::Symbol(first.chars().next().unwrap()));
        } else {
            tokens.push(Token::Identifier(first.to_string()));
        }
        for word in words {
            if let Ok(num) = word.parse::<i32>() {
                tokens.push(Token::Number(num));
            } else if word.len() == 1 && "{}()".contains(word) {
                tokens.push(Token::Symbol(word.chars().next().unwrap()));
            } else {
                tokens.push(Token::Identifier(word.to_string()));
            }
        }
    }
    tokens
}

/// Tokenizes a multi-line robot DSL script.
pub fn tokenize_script(script: &str) -> Vec<Token> {
    script
        .lines()
        .flat_map(|line| tokenize_line(line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_line_basic() {
        let line = "rotate treads 90";
        let tokens = tokenize_line(line);
        assert_eq!(
            tokens,
            vec![
                Token::Keyword("rotate".to_string()),
                Token::Identifier("treads".to_string()),
                Token::Number(90)
            ]
        );
    }

    #[test]
    fn test_tokenize_line_symbols() {
        let line = "if scan > 0 {";
        let tokens = tokenize_line(line);
        assert_eq!(
            tokens,
            vec![
                Token::Keyword("if".to_string()),
                Token::Identifier("scan".to_string()),
                Token::Identifier(">".to_string()),
                Token::Number(0),
                Token::Symbol('{')
            ]
        );
    }

    #[test]
    fn test_tokenize_script_multiline() {
        let script = r#"
rotate treads 90
move forward 10
scan
fire
if scan > 0 {
    fire
}
"#;
        let tokens = tokenize_script(script);
        let expected = vec![
            Token::Keyword("rotate".to_string()),
            Token::Identifier("treads".to_string()),
            Token::Number(90),
            Token::Keyword("move".to_string()),
            Token::Identifier("forward".to_string()),
            Token::Number(10),
            Token::Keyword("scan".to_string()),
            Token::Keyword("fire".to_string()),
            Token::Keyword("if".to_string()),
            Token::Identifier("scan".to_string()),
            Token::Identifier(">".to_string()),
            Token::Number(0),
            Token::Symbol('{'),
            Token::Keyword("fire".to_string()),
            Token::Symbol('}'),
        ];
        assert_eq!(tokens, expected);
    }
}
