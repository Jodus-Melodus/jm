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
    Token(TokenType, String),
}

pub const KEYWORDS: [&str; 5] = ["let", "if", "else", "while", "for"];

pub fn tokenize(source_code: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut number = String::new();
    let mut name = String::new();
    let mut parsing_number = false;

    for character in source_code.chars() {
        if !character.is_alphanumeric() && '.' != character {
            if !name.is_empty() {
                if KEYWORDS.contains(&name.as_str()) {
                    tokens.push(Token::Token(TokenType::Keyword, name));
                } else {
                    tokens.push(Token::Token(TokenType::Identifier, name));
                }
                name = String::new();
            }

            if !number.is_empty() {
                if number.contains('.') {
                    tokens.push(Token::Token(TokenType::Float, number));
                } else {
                    tokens.push(Token::Token(TokenType::Integer, number));
                }
                number = String::new();
                parsing_number = false;
            }
        }

        match character {
            ' ' | '\t' | '\n' | '\r' => continue,
            '=' => tokens.push(Token::Token(
                TokenType::AssignmentOperator,
                String::from(character),
            )),
            '+' | '-' | '*' | '/' | '%' | '^' => tokens.push(Token::Token(
                TokenType::BinaryOperator,
                String::from(character),
            )),
            '(' => tokens.push(Token::Token(
                TokenType::OpenParenthesis,
                String::from(character),
            )),
            ')' => tokens.push(Token::Token(
                TokenType::CloseParenthesis,
                String::from(character),
            )),
            '[' => tokens.push(Token::Token(
                TokenType::OpenBracket,
                String::from(character),
            )),
            ']' => tokens.push(Token::Token(
                TokenType::CloseBracket,
                String::from(character),
            )),
            '{' => tokens.push(Token::Token(TokenType::OpenBrace, String::from(character))),
            '}' => tokens.push(Token::Token(TokenType::CloseBrace, String::from(character))),
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
                    tokens.push(Token::Token(TokenType::Dot, String::from(character)));
                }
            }
            _ => return Err(format!("Invalid character found: '{:?}'", character)),
        }
    }

    if !name.is_empty() {
        if KEYWORDS.contains(&name.as_str()) {
            tokens.push(Token::Token(TokenType::Keyword, name));
        } else {
            tokens.push(Token::Token(TokenType::Identifier, name));
        }
    }

    if !number.is_empty() {
        if number.contains('.') {
            tokens.push(Token::Token(TokenType::Float, number));
        } else {
            tokens.push(Token::Token(TokenType::Integer, number));
        }
    }

    tokens.push(Token::Token(TokenType::EOF, String::new()));
    Ok(tokens)
}
