use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub enum RuntimeValue {
    Null,
    Integer(i128),
    Float(f64),
    String(String),
    Boolean(bool),
    Array(Vec<RuntimeValue>),
    Iterable(Vec<Node>),
    Function {
        args: Vec<RuntimeValue>,
        body: Vec<Node>,
    },
    NativeFunction {
        args: Vec<RuntimeValue>,
        function_call: fn(Vec<RuntimeValue>) -> RuntimeValue,
    },
}

impl Display for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Null => write!(f, "NULL"),
            RuntimeValue::Integer(i) => write!(f, "{}", i),
            RuntimeValue::Float(r) => write!(f, "{}", r),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Array(_) => todo!(),
            RuntimeValue::Iterable(_) => todo!(),
            RuntimeValue::Function { args: _, body: _ } => todo!(),
            RuntimeValue::NativeFunction {
                args: _,
                function_call: _,
            } => todo!(),
        }
    }
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
            Node::AssignmentExpression { name, value } => format!(
                "
                {{
                    \"kind\": \"assignment expression\",
                    \"name\": \"{:?}\",
                    \"value\": {:?}
                }}",
                name, value
            ),
            Node::VariableDeclaration { name, value } => format!(
                "
                {{
                    \"kind\": \"variable declaration\",
                    \"name\": \"{:?}\",
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
        };
        f.write_str(&value)
    }
}
