use crate::{
    error::{Error, ErrorType},
    lexer::{Token, TokenType},
};
use core::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, Clone)]
pub enum Node {
    StringLiteral(String),
    FloatLiteral(f64),
    IntegerLiteral(i128),
    Identifier(String),
    BinaryExpression {
        left: Box<Node>,
        operand: char,
        right: Box<Node>,
    },
    AssignmentExpression {
        name: Box<Node>,
        value: Box<Node>,
    },
    VariableDeclaration {
        name: Box<Node>,
        value: Box<Node>,
    },
    Scope {
        body: Vec<Node>,
    },
}

pub fn generate_ast(tokens: Vec<Token>) -> (Node, Vec<Error>) {
    let mut program = Vec::new();
    let mut tokens = tokens.into_iter().peekable();
    let mut errors = Vec::new();

    loop {
        if let Some(Token::Token { token_type, .. }) = tokens.peek().cloned() {
            match token_type {
                TokenType::EOF => break,
                _ => (),
            }

            let result = parse(&mut tokens);
            match result {
                Ok(expr) => program.push(expr),
                Err(err) => errors.push(err),
            }
        }
    }

    (Node::Scope { body: program }, errors)
}

fn parse(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    if let Some(Token::Token { token_type, .. }) = tokens.peek() {
        match token_type {
            TokenType::Keyword => parse_statement(tokens),
            _ => parse_expression(tokens),
        }
    } else {
        Err(Error::new(
            ErrorType::SyntaxError,
            format!("Expected token"),
            0,
            0,
        ))
    }
}

fn parse_statement(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    if let Some(Token::Token { value, .. }) = tokens.peek() {
        match value.as_str() {
            "let" => parse_variable_declaration_expression(tokens),
            _ => Err(Error::new(
                ErrorType::NameError,
                format!("Found unknown keyword '{}'", value),
                0,
                0,
            )),
        }
    } else {
        Err(Error::new(
            ErrorType::SyntaxError,
            format!("Expected token"),
            0,
            0,
        ))
    }
}

fn parse_variable_declaration_expression(
    tokens: &mut Peekable<IntoIter<Token>>,
) -> Result<Node, Error> {
    tokens.next();

    let assignment = parse_assignment_expression(tokens)?;
    match assignment {
        Node::AssignmentExpression { name, value } => Ok(Node::VariableDeclaration { name, value }),
        _ => Err(Error::new(
            ErrorType::SyntaxError,
            format!("Expected variable assignment"),
            0,
            0,
        )),
    }
}

fn parse_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    parse_assignment_expression(tokens)
}

fn parse_assignment_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    let left = parse_additive_expression(tokens)?;

    if let Some(Token::Token { token_type, .. }) = tokens.peek() {
        match token_type {
            TokenType::AssignmentOperator => {
                tokens.next();
                let value = parse_additive_expression(tokens)?;

                Ok(Node::AssignmentExpression {
                    name: Box::new(left),
                    value: Box::new(value),
                })
            }
            _ => Ok(left),
        }
    } else {
        Ok(left)
    }
}

fn parse_additive_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    let mut left = parse_multiplicative_expression(tokens)?;

    while let Some(Token::Token { value, .. }) = tokens.peek().cloned() {
        if ["+", "-"].contains(&value.as_str()) {
            tokens.next();
            let operand = value.chars().next().unwrap();
            let right = parse_multiplicative_expression(tokens)?;
            left = Node::BinaryExpression {
                left: Box::new(left),
                operand,
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_multiplicative_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    let mut left = parse_primary_expression(tokens)?;

    while let Some(Token::Token { value, .. }) = tokens.peek().cloned() {
        if ["*", "/", "%", "^"].contains(&value.as_str()) {
            tokens.next();
            let operand = value.chars().next().unwrap();
            let right = parse_primary_expression(tokens)?;
            left = Node::BinaryExpression {
                left: Box::new(left),
                operand,
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Ok(left)
}

fn parse_primary_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    if let Some(Token::Token {
        token_type,
        value,
        line,
        column,
    }) = tokens.next()
    {
        match token_type {
            TokenType::Integer => Ok(Node::IntegerLiteral(value.parse::<i128>().unwrap())),
            TokenType::Float => Ok(Node::FloatLiteral(value.parse::<f64>().unwrap())),
            TokenType::Identifier => Ok(Node::Identifier(value.to_string())),
            TokenType::OpenParenthesis => {
                let node = parse_expression(tokens)?;

                if let Some(Token::Token {
                    token_type,
                    value,
                    line,
                    column,
                }) = tokens.next()
                {
                    match token_type {
                        TokenType::CloseParenthesis => Ok(node),
                        _ => Err(Error::new(
                            ErrorType::SyntaxError,
                            format!("Expected a ')' found '{}'", value),
                            line,
                            column,
                        )),
                    }
                } else {
                    Err(Error::new(
                        ErrorType::SyntaxError,
                        format!("Expected a ')'"),
                        0,
                        0,
                    ))
                }
            }
            _ => Err(Error::new(
                ErrorType::SyntaxError,
                format!("Unexpected token '{}'", value),
                line,
                column,
            )),
        }
    } else {
        Err(Error::new(
            ErrorType::SyntaxError,
            format!("Expected a token."),
            0,
            0,
        ))
    }
}
