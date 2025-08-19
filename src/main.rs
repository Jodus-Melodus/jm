use std::{
    env,
    fs::File,
    io::{self, Read, Write},
};

use error::Error;

pub mod environment;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod native_functions;
pub mod parser;
pub mod types;

fn read_line(prompt: &str) -> String {
    let mut buffer = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn write_file(path: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn run_program(path: &str) -> Result<(), Error> {
    let source_code = read_file(path).unwrap();
    let mut environment = environment::generate_environment();

    let tokens = lexer::tokenize(&source_code)?;
    let (ast, errors) = parser::generate_ast(tokens);
    if errors.len() > 0 {
        for error in errors {
            println!("{}", error);
        }
        return Ok(());
    }

    write_file("ast.json", &format!("{:?}", ast)).unwrap();

    interpreter::evaluate(ast, &mut environment)?;
    Ok(())
}

fn program_loop() -> Result<(), Error> {
    let mut source_code = String::from(' ');
    let mut environment = environment::generate_environment();

    while !source_code.is_empty() {
        source_code = read_line("> ");
        let tokens = lexer::tokenize(&source_code)?;
        let (ast, errors) = parser::generate_ast(tokens);
        if errors.len() > 0 {
            for error in errors {
                println!("{}", error);
            }
            return Ok(());
        }

        let result = interpreter::evaluate(ast, &mut environment)?;

        println!("{:?}", result);
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let arguments = env::args().collect::<Vec<String>>()[1..].to_vec();

    if arguments.len() > 0 {
        let file_path = arguments[0].as_str();
        run_program(file_path)?;
    } else {
        program_loop()?;
    }

    Ok(())
}
