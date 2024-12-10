use std::collections::HashMap;

pub mod interpreter;
pub mod lexer;
pub mod parser;

fn main() -> Result<(), String> {
    let source_code = "let a = (3 + 5)";

    let tokens = lexer::tokenize(source_code)?;
    let ast = parser::generate_ast(tokens)?;
    let mut environment = HashMap::new();
    let result = interpreter::evaluate(ast, &mut environment)?;

    println!("{:?}", result);

    Ok(())
}
