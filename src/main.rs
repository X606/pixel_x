use std::io;

use crate::parser::PrettyPrintable;

mod types;
mod lexer;
mod parser;

fn main() {
    println!("Please input something to be processed :)");
    let mut value = String::new();
    
    let _read_line = io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let lexer_output = match lexer::lexer(value) {
        Ok(value) => value,
        Err(err) => {
            println!("Lexer error: {}", err);
            return;
        }
    };

    let parser_output = match parser::parser(lexer_output) {
        Ok(value) => value,
        Err(err) => {
            println!("Parser error: {}", err);
            return;
        }
    };

    println!("Output:\n{:?}", Box::<dyn PrettyPrintable>::from(parser_output));
}
