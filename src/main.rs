mod json_value;
mod lexer;
mod parser;
use std::{env, fs, io};

use json_value::JsonValue;
use lexer::*;
use parser::*;

fn parse_json(json_data: String) -> Option<JsonValue> {
    let tokens = Lexer::new(json_data).lex()?;
    let json = Parser::new(tokens).parse()?;
    Some(json)
}

#[derive(Debug)]
enum JsocksError {
    NoFileFoundErr,
    IOError(io::Error),
}

fn main() -> Result<(), JsocksError> {
    let Some(json_path) = env::args().nth(1) else {
        eprintln!("No JSON file provided");
        return Err(JsocksError::NoFileFoundErr);
    };
    let data = fs::read_to_string(json_path).map_err(|e| JsocksError::IOError(e))?;

    if let Some(json) = parse_json(data) {
        println!("{}", json.dump());
    } else {
        println!("Failed to parse JSON");
    }
    Ok(())
}
