use std::collections::HashMap;

use crate::{
    error::{Error, ErrorType},
    native_functions::nf_print,
    types::RuntimeValue,
};

pub fn generate_environment() -> HashMap<String, RuntimeValue> {
    let mut environment = HashMap::new();

    environment.insert(
        "print".to_string(),
        RuntimeValue::NativeFunction {
            args: vec![],
            function_call: nf_print,
        },
    );

    environment
}

pub fn declare(
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

pub fn assign(
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

pub fn lookup(hashmap: &mut HashMap<String, RuntimeValue>, name: String) -> Option<RuntimeValue> {
    hashmap.get(&name).cloned()
}
