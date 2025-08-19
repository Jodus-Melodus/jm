use crate::types::RuntimeValue;

pub fn nf_print(args: RuntimeValue) -> RuntimeValue {
    if let RuntimeValue::Array(values) = args {
        println!(
            "{}",
            values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    RuntimeValue::Null
}
