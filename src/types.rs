use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Rem, Sub},
};

#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Null,
    Integer(i128),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<RuntimeValue>),
    Iterable(Vec<Node>),
    Function(Vec<RuntimeValue>, Vec<Node>),
    NativeFunction(String, fn(RuntimeValue) -> RuntimeValue),
}

#[derive(Clone)]
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
        assignment_type: char,
        value: Box<Node>,
    },
    VariableDeclaration {
        name: Box<Node>,
        value: Box<Node>,
    },
    Scope {
        body: Vec<Node>,
    },
    Arguments(Vec<Node>),
    FunctionCall {
        name: Box<Node>,
        arguments: Box<Node>,
    },
}

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
    Comma,
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

#[derive(Debug)]
pub enum ErrorType {
    Error,
    SyntaxError,
    NameError,
    TypeError,
}

pub struct Error {
    error_type: ErrorType,
    message: String,
    line: u128,
    column: u128,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String, line: u128, column: u128) -> Self {
        Error {
            error_type,
            message,
            line,
            column,
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: {} in line {} column {}",
            self.error_type, self.message, self.line, self.column
        )
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error {{ error_type: {:?}, message: {}, line: {}, column: {} }}",
            self.error_type, self.message, self.line, self.column
        )
    }
}

impl Default for Error {
    fn default() -> Self {
        Error {
            error_type: ErrorType::Error,
            message: format!(""),
            line: 0,
            column: 0,
        }
    }
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Null => write!(f, "NULL"),
            RuntimeValue::Integer(i) => write!(f, "{}", i),
            RuntimeValue::Float(r) => write!(f, "{}", r),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Array(a) => {
                write!(
                    f,
                    "{}",
                    a.iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            RuntimeValue::Iterable(_) => todo!(),
            RuntimeValue::Function(_, _) => todo!(),
            RuntimeValue::NativeFunction(name, _) => write!(f, "Native Function: {}", name),
        }
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Node::StringLiteral(s) => format!(
                "
                {{
                    \"kind\": \"string literal\",
                    \"value\": \"{}\"
                }}",
                s
            ),
            Node::FloatLiteral(f) => format!(
                "
                {{
                    \"kind\": \"float literal\",
                    \"value\": {}
                }}",
                f
            ),
            Node::IntegerLiteral(i) => format!(
                "
                {{
                    \"kind\": \"integer literal\",
                    \"value\": {}
                }}",
                i
            ),
            Node::Identifier(i) => format!(
                "
                {{
                    \"kind\": \"identifier\",
                    \"value\": \"{}\"
                }}",
                i
            ),
            Node::BinaryExpression {
                left,
                operand,
                right,
            } => format!(
                "
                {{
                    \"kind\": \"binary expression\",
                    \"left\": {:?},
                    \"operand\": \"{}\",
                    \"right\": {:?}
                }}",
                left, operand, right
            ),
            Node::AssignmentExpression {
                name,
                assignment_type,
                value,
            } => format!(
                "
                {{
                    \"kind\": \"assignment expression\",
                    \"name\": {:?},
                    \"assignment_type\": {}
                    \"value\": {:?}
                }}",
                name, assignment_type, value
            ),
            Node::VariableDeclaration { name, value } => format!(
                "
                {{
                    \"kind\": \"variable declaration\",
                    \"name\": {:?},
                    \"value\": {:?}
                }}",
                name, value
            ),
            Node::Scope { body } => format!(
                "
                {{
                    \"kind\": \"scope\",
                    \"body\": {:?}
                }}",
                body
            ),
            Node::FunctionCall { name, arguments } => format!(
                "
                {{
                    \"kind\": \"function call\",
                    \"name\": {:?},
                    \"arguments\": {:?}

                }}",
                name, arguments
            ),
            Node::Arguments(nodes) => format!("{:?}", nodes),
        };
        f.write_str(&value)
    }
}

impl Add for RuntimeValue {
    type Output = RuntimeValue;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => RuntimeValue::Integer(l + r),
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l as f64 + r),
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => RuntimeValue::Float(l + r as f64),
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l + r),
            _ => todo!(),
        }
    }
}

impl Sub for RuntimeValue {
    type Output = RuntimeValue;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => RuntimeValue::Integer(l - r),
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l as f64 - r),
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => RuntimeValue::Float(l - r as f64),
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l - r),
            _ => todo!(),
        }
    }
}

impl Mul for RuntimeValue {
    type Output = RuntimeValue;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => RuntimeValue::Integer(l * r),
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l as f64 * r),
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => RuntimeValue::Float(l * r as f64),
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l * r),
            _ => todo!(),
        }
    }
}

impl Div for RuntimeValue {
    type Output = RuntimeValue;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                RuntimeValue::Float(l as f64 / r as f64)
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l as f64 / r),
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => RuntimeValue::Float(l / r as f64),
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => RuntimeValue::Float(l / r),
            _ => todo!(),
        }
    }
}
impl Rem for RuntimeValue {
    type Output = RuntimeValue;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => RuntimeValue::Integer(l % r),
            _ => todo!(),
        }
    }
}
