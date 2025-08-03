// Parser for the robot-battle DSL.
// Converts a stream of tokens into an AST (Vec<Command>).
//
// Supports: move, rotate, scan, fire, loop { ... }

use crate::ast::{Command, Section};
use crate::tokenizer::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    #[allow(dead_code)]
    UnexpectedToken(Token),
    InvalidCommand,
}

/// Parse a script (token stream) into a sequence of commands (AST).
pub fn parse_tokens(tokens: &[Token]) -> Result<Vec<Command>, ParseError> {
    let mut idx = 0;
    let mut commands = Vec::new();

    while idx < tokens.len() {
        match &tokens[idx] {
            Token::Keyword(k) if k == "move" => {
                // move <direction> <distance>
                idx += 1;
                let direction = match tokens.get(idx) {
                    Some(Token::Identifier(dir)) => dir.clone(),
                    Some(tok) => return Err(ParseError::UnexpectedToken(tok.clone())),
                    None => return Err(ParseError::UnexpectedEOF),
                };
                idx += 1;
                let distance = match tokens.get(idx) {
                    Some(Token::Number(n)) => *n,
                    Some(tok) => return Err(ParseError::UnexpectedToken(tok.clone())),
                    None => return Err(ParseError::UnexpectedEOF),
                };
                idx += 1;
                commands.push(Command::Move {
                    direction,
                    distance,
                });
            }
            Token::Keyword(k) if k == "rotate" => {
                // rotate <section> <angle>
                idx += 1;
                let section = match tokens.get(idx) {
                    Some(Token::Keyword(k)) if k == "body" => Section::Body,
                    Some(Token::Keyword(k)) if k == "turret" => Section::Turret,
                    Some(Token::Keyword(k)) if k == "scanner" => Section::Scanner,
                    Some(tok) => return Err(ParseError::UnexpectedToken(tok.clone())),
                    None => return Err(ParseError::UnexpectedEOF),
                };
                idx += 1;
                let angle = match tokens.get(idx) {
                    Some(Token::Number(n)) => *n,
                    Some(tok) => return Err(ParseError::UnexpectedToken(tok.clone())),
                    None => return Err(ParseError::UnexpectedEOF),
                };
                idx += 1;
                commands.push(Command::Rotate { section, angle });
            }
            Token::Keyword(k) if k == "scan" => {
                idx += 1;
                commands.push(Command::Scan);
            }
            Token::Keyword(k) if k == "fire" => {
                idx += 1;
                commands.push(Command::Fire);
            }
            Token::Keyword(k) if k == "loop" => {
                idx += 1;
                // Expect '{'
                match tokens.get(idx) {
                    Some(Token::Symbol('{')) => idx += 1,
                    Some(tok) => return Err(ParseError::UnexpectedToken(tok.clone())),
                    None => return Err(ParseError::UnexpectedEOF),
                }
                // Parse block until matching '}'
                let mut block = Vec::new();
                while idx < tokens.len() {
                    match &tokens[idx] {
                        Token::Symbol('}') => {
                            idx += 1;
                            break;
                        }
                        _ => {
                            // Recursively parse commands inside the block
                            let start = idx;
                            // Parse one command
                            match parse_tokens(&tokens[start..]) {
                                Ok(mut inner_cmds) if !inner_cmds.is_empty() => {
                                    // Only take the first command parsed
                                    block.push(inner_cmds.remove(0));
                                    // Advance idx by the number of tokens consumed for that command
                                    idx += tokens_consumed_for_command(&tokens[start..]);
                                }
                                Ok(_) => break,
                                Err(e) => return Err(e),
                            }
                        }
                    }
                }
                commands.push(Command::Loop { block });
            }
            Token::Symbol('}') | Token::Symbol('{') => {
                // Block delimiters are handled in loop parsing, skip them here
                idx += 1;
            }
            Token::Keyword(_) => {
                return Err(ParseError::InvalidCommand);
            }
            _ => {
                return Err(ParseError::UnexpectedToken(tokens[idx].clone()));
            }
        }
    }

    Ok(commands)
}

/// Helper function: returns the number of tokens consumed for a single command.
/// Used to advance the index when parsing blocks.
fn tokens_consumed_for_command(tokens: &[Token]) -> usize {
    if tokens.is_empty() {
        return 0;
    }
    match &tokens[0] {
        Token::Keyword(k) if k == "move" || k == "rotate" => 3,
        Token::Keyword(k) if k == "scan" || k == "fire" => 1,
        Token::Keyword(k) if k == "loop" => {
            // Find matching '{' and '}'
            let mut count = 1; // "loop"
            if tokens.get(count) == Some(&Token::Symbol('{')) {
                count += 1;
            }
            let mut depth = 1;
            while count < tokens.len() && depth > 0 {
                match &tokens[count] {
                    Token::Symbol('{') => depth += 1,
                    Token::Symbol('}') => depth -= 1,
                    _ => {}
                }
                count += 1;
            }
            count
        }
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Command;
    use crate::tokenizer::tokenize_script;

    #[test]
    fn test_parse_simple_commands() {
        let script = r#"
            move forward 5
            rotate turret 90
            scan
            fire
        "#;
        let tokens = tokenize_script(script);
        let ast = parse_tokens(&tokens).unwrap();
        assert_eq!(
            ast,
            vec![
                Command::Move {
                    direction: "forward".to_string(),
                    distance: 5
                },
                Command::Rotate {
                    section: Section::Turret,
                    angle: 90
                },
                Command::Scan,
                Command::Fire,
            ]
        );
    }

    #[test]
    fn test_parse_loop_block() {
        let script = r#"
            loop {
                scan
                move forward 1
                fire
            }
        "#;
        let tokens = tokenize_script(script);
        let ast = parse_tokens(&tokens).unwrap();
        assert_eq!(
            ast,
            vec![Command::Loop {
                block: vec![
                    Command::Scan,
                    Command::Move {
                        direction: "forward".to_string(),
                        distance: 1
                    },
                    Command::Fire,
                ]
            }]
        );
    }

    #[test]
    fn test_parse_nested_loops() {
        let script = r#"
            loop {
                loop {
                    scan
                }
                fire
            }
        "#;
        let tokens = tokenize_script(script);
        let ast = parse_tokens(&tokens).unwrap();
        assert_eq!(
            ast,
            vec![Command::Loop {
                block: vec![
                    Command::Loop {
                        block: vec![Command::Scan,]
                    },
                    Command::Fire,
                ]
            }]
        );
    }
}
