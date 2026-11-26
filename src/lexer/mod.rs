use crate::lexer::kind::Kind;
use crate::lexer::token::Token;

pub mod kind;
pub mod token;

pub fn tokenize(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = source.char_indices().peekable();

    while let Some(&(index, char)) = chars.peek() {
        assert!(char.is_ascii(), "Only ascii chars are valid!");

        match char {
            char if char.is_ascii_whitespace() => {
                chars.next();
            }
            char if char.is_ascii_alphabetic() => {
                let mut word = String::new();

                while let Some(&(_, char)) = chars.peek() {
                    if char.is_ascii_alphanumeric() || char == '_' {
                        word.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let end = index + word.len();
                tokens.push(Token::new(Kind::Identifier(word), index, end));
            }
            char if char.is_ascii_digit() => {
                let mut number = String::new();
                let mut dots = 0;

                while let Some(&(_, char)) = chars.peek() {
                    if char.is_ascii_digit() {
                        number.push(char);
                        chars.next();
                    } else if char == '.' {
                        dots += 1;
                        number.push(char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if dots > 1 || number.ends_with(".") {
                    panic!("Invalid number!")
                }

                let end = index + number.len();
                tokens.push(Token::new(Kind::Number(number), index, end));
            }
            _ => panic!("Unexpected character!"),
        }
    }

    tokens
}
