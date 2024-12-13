#[derive(Debug, Clone)]
pub enum TokenType {
    BinaryOperator,
    Float,
    Integer,
    Identifier,
    Dot,
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Keyword,
    AssignmentOperator,
    EOF,
}

#[derive(Debug, Clone)]
pub enum Token {
    Token {
        token_type: TokenType,
        value: String,
        line: u128,
        column: u128,
    },
}

pub const KEYWORDS: [&str; 5] = ["let", "if", "else", "while", "for"];

pub fn tokenize(source_code: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut number = String::new();
    let mut name = String::new();
    let mut parsing_number = false;
    let mut line = 1;
    let mut column = 1;

    for character in source_code.chars() {
        if !character.is_alphanumeric() && '.' != character {
            if !name.is_empty() {
                if KEYWORDS.contains(&name.as_str()) {
                    tokens.push(Token::Token {
                        token_type: TokenType::Keyword,
                        value: name,
                        line,
                        column,
                    });
                } else {
                    tokens.push(Token::Token {
                        token_type: TokenType::Identifier,
                        value: name,
                        line,
                        column,
                    });
                }
                name = String::new();
            }

            if !number.is_empty() {
                if number.contains('.') {
                    tokens.push(Token::Token {
                        token_type: TokenType::Float,
                        value: number,
                        line,
                        column,
                    });
                } else {
                    tokens.push(Token::Token {
                        token_type: TokenType::Integer,
                        value: number,
                        line,
                        column,
                    });
                }
                number = String::new();
                parsing_number = false;
            }
        }

        match character {
            ' ' | '\t' => continue,
            '\n' | '\r' => {
                line += 1;
                column = 1;
            }
            '=' => tokens.push(Token::Token {
                token_type: TokenType::AssignmentOperator,
                value: String::from(character),
                line,
                column,
            }),
            '+' | '-' | '*' | '/' | '%' | '^' => tokens.push(Token::Token {
                token_type: TokenType::BinaryOperator,
                value: String::from(character),
                line,
                column,
            }),
            '(' => tokens.push(Token::Token {
                token_type: TokenType::OpenParenthesis,
                value: String::from(character),
                line,
                column,
            }),
            ')' => tokens.push(Token::Token {
                token_type: TokenType::CloseParenthesis,
                value: String::from(character),
                line,
                column,
            }),
            '[' => tokens.push(Token::Token {
                token_type: TokenType::OpenBracket,
                value: String::from(character),
                line,
                column,
            }),
            ']' => tokens.push(Token::Token {
                token_type: TokenType::CloseBracket,
                value: String::from(character),
                line,
                column,
            }),
            '{' => tokens.push(Token::Token {
                token_type: TokenType::OpenBrace,
                value: String::from(character),
                line,
                column,
            }),
            '}' => tokens.push(Token::Token {
                token_type: TokenType::OpenBrace,
                value: String::from(character),
                line,
                column,
            }),
            'a'..='z' | 'A'..='Z' | '_' => {
                name.push(character);
                continue;
            }
            '0'..='9' => {
                number.push(character);
                parsing_number = true;
                continue;
            }
            '.' => {
                if parsing_number {
                    if number.contains('.') {
                        return Err(format!(
                            "Syntax Error: Number cannot contain more than one decimal."
                        ));
                    }

                    number.push(character);
                    continue;
                } else {
                    tokens.push(Token::Token {
                        token_type: TokenType::Dot,
                        value: String::from(character),
                        line,
                        column,
                    });
                }
            }
            _ => return Err(format!("Invalid character found: '{:?}'", character)),
        }
        column += 1;
    }

    if !name.is_empty() {
        if KEYWORDS.contains(&name.as_str()) {
            tokens.push(Token::Token {
                token_type: TokenType::Keyword,
                value: name,
                line,
                column,
            });
        } else {
            tokens.push(Token::Token {
                token_type: TokenType::Identifier,
                value: name,
                line,
                column,
            });
        }
    }

    if !number.is_empty() {
        if number.contains('.') {
            tokens.push(Token::Token {
                token_type: TokenType::Float,
                value: number,
                line,
                column,
            });
        } else {
            tokens.push(Token::Token {
                token_type: TokenType::Integer,
                value: number,
                line,
                column,
            });
        }
    }

    tokens.push(Token::Token {
        token_type: TokenType::EOF,
        value: String::new(),
        line,
        column,
    });
    Ok(tokens)
}
