use std::collections::HashMap;

use crate::{
    native_functions::nf_print,
    types::{Error, ErrorType, RuntimeValue},
};

pub fn generate_environment() -> HashMap<String, RuntimeValue> {
    let mut environment = HashMap::new();

    environment.insert(
        "print".to_string(),
        RuntimeValue::NativeFunction("print".to_string(), nf_print),
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
) -> Result<RuntimeValue, Error> {
    if hashmap.contains_key(&name) {
        hashmap.insert(name, value.clone());
        Ok(value)
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
