use crate::{
    error::{Error, ErrorType},
    native_functions::nf_print,
    parser::Node,
};
use std::{
    collections::HashMap,
    fmt::{write, Debug},
};

#[derive(Clone)]
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
        function_call: std::sync::Arc<dyn Fn(RuntimeValue)>,
    },
}

impl Debug for RuntimeValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeValue::Null => write!(f, "NULL"),
            RuntimeValue::Integer(i) => write!(f, "{}", i),
            RuntimeValue::Float(r) => write!(f, "{}", r),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::Array(values) => write!(f, "{:?}", values),
            RuntimeValue::Iterable(nodes) => todo!(),
            RuntimeValue::Function { args, body } => write!(f, "{:?} -> {:?}", args, body),
            RuntimeValue::NativeFunction {
                args,
                function_call,
            } => write!(f, "{:?} -> function_call", args),
        }
    }
}

pub fn generate_environment() -> HashMap<String, RuntimeValue> {
    let mut environment = HashMap::new();

    environment.insert(
        "print".to_string(),
        RuntimeValue::NativeFunction {
            args: vec![],
            function_call: std::sync::Arc::new(|x| {
                nf_print(vec![x]);
            }),
        },
    );

    environment
}

fn declare(
    hashmap: &mut HashMap<String, RuntimeValue>,
    name: String,
    value: RuntimeValue,
) -> Result<(), Error> {
    if hashmap.contains_key(&name) {
        Err(Error::new(
            ErrorType::NameError,
            format!("Variable '{}' already declared", name),
            0,
            0,
        ))
    } else {
        hashmap.insert(name, value);
        Ok(())
    }
}

fn assign(
    hashmap: &mut HashMap<String, RuntimeValue>,
    name: String,
    value: RuntimeValue,
) -> Result<(), Error> {
    if hashmap.contains_key(&name) {
        hashmap.insert(name, value);
        Ok(())
    } else {
        Err(Error::new(
            ErrorType::NameError,
            format!("'{}' is undefined", name),
            0,
            0,
        ))
    }
}

fn lookup(hashmap: &mut HashMap<String, RuntimeValue>, name: String) -> Option<RuntimeValue> {
    hashmap.get(&name).cloned()
}

pub fn evaluate(
    node: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    match node {
        Node::Scope { body: statements } => {
            let mut result = RuntimeValue::Null;
            for statement in statements {
                result = evaluate(statement, env)?;
            }

            Ok(result)
        }
        Node::IntegerLiteral(i) => Ok(RuntimeValue::Integer(i)),
        Node::StringLiteral(s) => Ok(RuntimeValue::String(s)),
        Node::FloatLiteral(f) => Ok(RuntimeValue::Float(f)),
        Node::Identifier(name) => evaluate_identifier(name, env),
        Node::BinaryExpression {
            left,
            operand,
            right,
        } => evaluate_binary_expression(*left, operand, *right, env),
        Node::AssignmentExpression { name, value } => {
            evaluate_assignment_expression(*name, *value, env)
        }
        Node::VariableDeclaration { name, value } => {
            evaluate_variable_declaration(*name, *value, env)
        }
    }
}

fn evaluate_identifier(
    name: String,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    let result = lookup(env, name.clone());
    match result {
        Some(value) => Ok(value.clone()),
        None => Err(Error::new(
            ErrorType::NameError,
            format!("'{}' is undefined", name),
            0,
            0,
        )),
    }
}

fn evaluate_variable_declaration(
    name: Node,
    value: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    if let Node::Identifier(name) = name {
        let value = evaluate(value, env)?;
        let res = declare(env, name, value.clone());
        match res {
            Err(e) => Err(e),
            Ok(_) => Ok(value),
        }
    } else {
        Err(Error::new(
            ErrorType::Error,
            format!("Expected a string value"),
            0,
            0,
        ))
    }
}

fn evaluate_assignment_expression(
    name: Node,
    value: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    if let Node::Identifier(name) = name {
        let value = evaluate(value, env)?;
        let res = assign(env, name, value.clone());
        match res {
            Err(e) => Err(e),
            Ok(_) => Ok(value),
        }
    } else {
        Err(Error::new(
            ErrorType::Error,
            format!("Expected a string value, found '{:?}'", name),
            0,
            0,
        ))
    }
}

fn evaluate_binary_expression(
    left: Node,
    operand: char,
    right: Node,
    environment: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    let left = evaluate(left, environment)?;
    let right = evaluate(right, environment)?;

    match operand {
        '+' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l + r))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 + r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l + r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l + r)),
            _ => Err(Error::new(
                ErrorType::TypeError,
                format!("Incompatible types: '{:?}' and '{:?}'", left, right),
                0,
                0,
            )),
        },
        '-' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l - r))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 - r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l - r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l - r)),
            _ => Err(Error::new(
                ErrorType::TypeError,
                format!("Incompatible types: '{:?}' and '{:?}'", left, right),
                0,
                0,
            )),
        },
        '*' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l * r))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 * r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l * r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l * r)),
            _ => Err(Error::new(
                ErrorType::TypeError,
                format!("Incompatible types: '{:?}' and '{:?}'", left, right),
                0,
                0,
            )),
        },
        '/' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l as f64 / r as f64))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float(l as f64 / r))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l / r as f64))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l / r)),
            _ => Err(Error::new(
                ErrorType::TypeError,
                format!("Incompatible types: '{:?}' and '{:?}'", left, right),
                0,
                0,
            )),
        },
        '%' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l % r))
            }
            _ => Err(Error::new(
                ErrorType::TypeError,
                format!("Incompatible types: '{:?}' and '{:?}'", left, right),
                0,
                0,
            )),
        },
        '^' => match (left.clone(), right.clone()) {
            (RuntimeValue::Integer(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Integer(l.pow(r.try_into().unwrap())))
            }
            (RuntimeValue::Integer(l), RuntimeValue::Float(r)) => {
                Ok(RuntimeValue::Float((l as f64).powf(r)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Integer(r)) => {
                Ok(RuntimeValue::Float(l.powf(r as f64)))
            }
            (RuntimeValue::Float(l), RuntimeValue::Float(r)) => Ok(RuntimeValue::Float(l.powf(r))),
            _ => Err(Error::new(
                ErrorType::TypeError,
                format!("Incompatible types: '{:?}' and '{:?}'", left, right),
                0,
                0,
            )),
        },
        _ => Ok(RuntimeValue::Null),
    }
}
