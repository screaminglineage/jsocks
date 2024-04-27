mod lexer;
mod parser;
use std::{fs, io};

use lexer::*;
use parser::*;

fn main() -> io::Result<()> {
    let json_path = "test.json";
    let data = fs::read_to_string(json_path)?;
    let lexer = Lexer::new(data);
    let tokens = lexer.lex().unwrap();
    for token in &tokens {
        println!("{token:?}");
    }

    let mut parser = Parser::new(tokens);
    let json = parser.parse();
    println!("{json:#?}");

    Ok(())
}
