use std::{fs, io::Error};

#[derive(Debug)]
enum Kind {
    Identifier,
    Number
}

#[derive(Debug)]
struct Token {
    kind: Kind,
    value: String
}

impl Token {
    pub fn new(kind: Kind, value: String) -> Self {
        Token { kind, value }
    }
}

fn tokenize(source: String) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::<Token>::new();
    let mut chars = source.chars().peekable();

    while let Some(char) = chars.peek() {
        assert!(char.is_ascii(), "Only ascii chars are valid!");

        match char {
            char if char.is_ascii_alphabetic() => {
                let mut word = String::new();

                while let Some(&char) = chars.peek() {
                    if char.is_ascii_alphanumeric() || char == '_' {
                        word.push(char);
                        chars.next();
                    } else { break; }
                }

                tokens.push(Token::new(Kind::Identifier, word));
            }
            char if char.is_ascii_digit() => {
                let mut number = String::new();
                let mut dots = 0;

                while let Some(&char) = chars.peek() {
                    if char.is_ascii_digit() {
                        number.push(char);
                        chars.next();
                    } else if char == '.' {
                        dots += 1;
                        number.push(char);
                    } else { break; }
                }

                if dots > 1 || number.ends_with(".") {
                    panic!("Invalid number!")
                }

                tokens.push(Token::new(Kind::Number, number));
            }
            char if char.is_ascii_whitespace() => { chars.next(); }
            _ => panic!("Unexpected character!")
        }
    }

    Ok(tokens)
}

fn main() {
    let tokens = match fs::read_to_string("main.probe") {
        Err(_) => panic!("Error while reading the source!"),
        Ok(src) => tokenize(src).unwrap()
    };

    println!("{:?}", tokens);
}
