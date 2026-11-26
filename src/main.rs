use std::fs;

pub mod lexer;

fn main() {
    let tokens = match fs::read_to_string("main.probe") {
        Err(_) => panic!("Error while reading the source!"),
        Ok(source) => lexer::tokenize(source),
    };

    println!("{:?}", tokens);
}
