mod lexer;
use std::{fs, io};

use lexer::*;

fn main() -> io::Result<()> {
    let json_path = "test.json";
    let data = fs::read_to_string(json_path)?;
    let lexer = Lexer::new(data);
    let tokens = lexer.lex().unwrap();
    for token in tokens {
        println!("{token:?}");
    }

    Ok(())
}
