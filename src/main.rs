use std::{collections::HashMap, io::Write};

pub mod interpreter;
pub mod lexer;
pub mod parser;

fn read_line(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn main() -> Result<(), String> {
    let mut source_code = String::from(' ');
    let mut environment = HashMap::new();

    while !source_code.is_empty() {
        source_code = read_line("> ");
        let tokens = lexer::tokenize(&source_code)?;
        let ast = parser::generate_ast(tokens)?;
        let result = interpreter::evaluate(ast, &mut environment)?;

        println!("{:?}", result);
    }

    Ok(())
}
