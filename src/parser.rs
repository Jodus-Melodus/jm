use crate::types::{Error, ErrorType, Node, Token, TokenType};
use core::iter::Peekable;
use std::vec::IntoIter;

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
        Node::AssignmentExpression { name, value, .. } => {
            Ok(Node::VariableDeclaration { name, value })
        }
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

fn parse_arguments(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    let Token::Token { token_type, .. } = tokens
        .next()
        .ok_or_else(|| Error::new(ErrorType::SyntaxError, format!("Expected a '('"), 0, 0))?;

    match token_type {
        TokenType::OpenParenthesis => {
            let Token::Token { token_type, .. } = tokens.peek().ok_or_else(|| {
                Error::new(
                    ErrorType::SyntaxError,
                    format!("Expected a value or ')'"),
                    0,
                    0,
                )
            })?;

            match token_type {
                TokenType::CloseParenthesis => Ok(Node::Arguments(vec![])),
                _ => Ok(Node::Arguments(parse_argument_list(tokens)?)),
            }
        }
        _ => Err(Error::new(
            ErrorType::SyntaxError,
            format!("Expected a '('"),
            0,
            0,
        )),
    }
}

fn parse_argument_list(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Vec<Node>, Error> {
    let mut arguments = vec![parse_expression(tokens)?];

    while let Some(Token::Token { token_type, .. }) = tokens.peek() {
        match token_type {
            TokenType::Comma => {
                tokens.next();

                if let Some(_) = tokens.peek() {
                    arguments.push(parse_expression(tokens)?);
                } else {
                    return Err(Error::new(
                        ErrorType::SyntaxError,
                        format!("Expected a value"),
                        0,
                        0,
                    ));
                }
            }
            _ => break,
        }
    }

    let Token::Token { token_type, .. } = tokens
        .peek()
        .ok_or_else(|| Error::new(ErrorType::SyntaxError, format!("Expected a ')'"), 0, 0))?;
    match token_type {
        TokenType::CloseParenthesis => {
            tokens.next();
            Ok(arguments)
        }
        _ => Err(Error::new(
            ErrorType::SyntaxError,
            format!("Expected a ')'"),
            0,
            0,
        )),
    }
}

fn parse_assignment_expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, Error> {
    let left = parse_additive_expression(tokens)?;

    if let Some(Token::Token {
        token_type, value, ..
    }) = tokens.peek().cloned()
    {
        match token_type {
            TokenType::AssignmentOperator => {
                tokens.next();
                let assignment_value = parse_additive_expression(tokens)?;

                Ok(Node::AssignmentExpression {
                    name: Box::new(left),
                    assignment_type: value.chars().nth(0).unwrap_or_else(|| '='),
                    value: Box::new(assignment_value),
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
            TokenType::Identifier => {
                if let Some(Token::Token { token_type, .. }) = tokens.peek() {
                    match token_type {
                        TokenType::OpenParenthesis => {
                            let args = parse_arguments(tokens)?;

                            Ok(Node::FunctionCall {
                                name: Box::new(Node::Identifier(value.to_string())),
                                arguments: Box::new(args),
                            })
                        }
                        _ => Ok(Node::Identifier(value.to_string())),
                    }
                } else {
                    Ok(Node::Identifier(value.to_string()))
                }
            }
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
            TokenType::OpenBrace => {
                let mut body = Vec::new();

                loop {
                    if let Some(Token::Token { token_type, .. }) = tokens.peek().cloned() {
                        match token_type {
                            TokenType::CloseBrace => {
                                tokens.next();
                                break;
                            }
                            _ => {
                                let result = parse(tokens);
                                match result {
                                    Ok(expr) => body.push(expr),
                                    Err(err) => return Err(err),
                                }
                            }
                        }
                    }
                }

                Ok(Node::Scope { body })
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
