use crate::types::RuntimeValue;

pub fn nf_print(args: Vec<RuntimeValue>) -> RuntimeValue {
    for arg in args {
        print!("{}", arg);
    }

    RuntimeValue::Null
}
