use crate::{
    environment,
    types::{Error, ErrorType, Node, RuntimeValue},
};
use std::collections::HashMap;

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
        Node::AssignmentExpression {
            name,
            assignment_type,
            value,
        } => evaluate_assignment_expression(*name, assignment_type, *value, env),
        Node::VariableDeclaration { name, value } => {
            evaluate_variable_declaration(*name, *value, env)
        }
        Node::Arguments(arguments) => evaluate_arguments(arguments, env),
        Node::FunctionCall { name, arguments } => evaluate_function_call(*name, *arguments, env),
    }
}

fn evaluate_function_call(
    name: Node,
    arguments: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    let func = evaluate(name, env)?;

    match func {
        RuntimeValue::NativeFunction(_, function_call) => {
            if let Node::Arguments(arguments) = arguments {
                let args = evaluate_arguments(arguments, env)?;
                Ok(function_call(args))
            } else {
                Err(Error::new(ErrorType::Error, format!(""), 0, 0))
            }
        }
        _ => Err(Error::new(ErrorType::Error, format!(""), 0, 0)),
    }
}

fn evaluate_arguments(
    args: Vec<Node>,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    let mut arguments = Vec::new();

    for argument in args {
        arguments.push(evaluate(argument, env)?);
    }

    Ok(RuntimeValue::Array(arguments))
}

fn evaluate_identifier(
    name: String,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    if let Some(result) = environment::lookup(env, name.clone()) {
        Ok(result)
    } else {
        Err(Error::new(
            ErrorType::NameError,
            format!("'{}' is undefined", name),
            0,
            0,
        ))
    }
}

fn evaluate_variable_declaration(
    name: Node,
    value: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    if let Node::Identifier(name) = name {
        let value = evaluate(value, env)?;
        let res = environment::declare(env, name, value.clone());
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
    assignment_type: char,
    value: Node,
    env: &mut HashMap<String, RuntimeValue>,
) -> Result<RuntimeValue, Error> {
    if let Node::Identifier(name) = name {
        let mut assignment_value = evaluate(value, env)?;
        let current_value = evaluate_identifier(name.clone(), env)?;
        if assignment_type != '=' {
            assignment_value =
                evaluate_calculation(current_value, assignment_type, assignment_value);
        }

        environment::assign(env, name, assignment_value.clone())
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

    Ok(evaluate_calculation(left, operand, right))
}

fn evaluate_calculation(left: RuntimeValue, operand: char, right: RuntimeValue) -> RuntimeValue {
    match operand {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        '%' => left % right,
        // '^' => left right,
        _ => RuntimeValue::Null,
    }
}
