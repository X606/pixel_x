use std::{iter::Peekable, str::Chars};
use crate::types::selector::Selector;

pub fn lexer(input: String) -> Result<Vec<LexerToken>,String> {
    let mut data = input.chars().peekable();
    let mut tokens: Vec<LexerToken> = Vec::new();

    loop {
        let current_char = match data.next() {
            Some(value) => value,
            None => {
                tokens.push(LexerToken::EndOfFile);
                break;
            }
        };

        match current_char {
            c if c.is_numeric() => {
                let mut current_string = String::from(current_char);
                read_until(&mut data, &mut current_string, |c| c.is_numeric() || c == '.');
                
                if current_string.contains('.') {
                    let f: f64 = match current_string.parse() {
                        Ok(value) => value,
                        Err(_) => return Err(String::from("Unable to parse float"))
                    };
                    tokens.push(LexerToken::LiteralFloat(f));
                }
                else {
                    let i: i64 = match current_string.parse() {
                        Ok(value) => value,
                        Err(_) => return Err(String::from("Unable to parse int"))
                    };
                    tokens.push(LexerToken::LiteralInt(i));
                }
            },
            c if c.is_alphabetic() => {
                let mut current_string = String::from(current_char);
                read_until(&mut data, &mut current_string, |c| c.is_alphanumeric() || c == '_');

                let token = lex_keyword(current_string);
                tokens.push(token);
            },
            c if c.is_whitespace() => {
                let mut current_string = String::from(current_char);
                read_until(&mut data, &mut current_string, |c| c.is_whitespace());

                tokens.push(LexerToken::Whitespace);
            },
            '(' => tokens.push(LexerToken::OpenParenthesis),
            ')' => tokens.push(LexerToken::CloseParenthesis),
            ';' => tokens.push(LexerToken::Semicolon),
            '+' => tokens.push(LexerToken::Plus),
            '-' => tokens.push(LexerToken::Dash),
            '*' => tokens.push(LexerToken::Star),
            '.' => tokens.push(LexerToken::Period),
            '/' => {
                let next_char = match data.peek() {
                    Some(val) => val.to_owned(),
                    None => '\0'
                };
                match next_char {
                    '/' => {
                        data.next().unwrap();
                        let mut body = String::new();
                        read_until(&mut data, &mut body, |c|!matches!(c,'\n'));
                        tokens.push(LexerToken::Comment(body));
                    },
                    _ => tokens.push(LexerToken::Slash)
                }
            },
            '!' => {
                let next_char = match data.peek() {
                    Some(val) => val.to_owned(),
                    None => '\0'
                };
                if next_char == '=' {
                    tokens.push(LexerToken::BangEquals);
                    data.next().unwrap();
                } else {
                    tokens.push(LexerToken::Bang);
                }
            },
            '=' => {
                let next_char = match data.peek() {
                    Some(val) => val.to_owned(),
                    None => '\0'
                };
                if next_char == '=' {
                    tokens.push(LexerToken::DoubleEquals);
                    data.next().unwrap();
                } else {
                    tokens.push(LexerToken::Equals);
                }
            }
            '@' => {
                let type_char = match data.next() {
                    Some(value) => value,
                    None => return Err(String::from("expected chacters following \"@\""))
                };
    
                let body = if matches!(data.peek(),Some('[')) {
                    data.next().unwrap();
    
                    let mut body = String::new();
    
                    read_until(&mut data, &mut body, |c|!matches!(c,']'));
                    data.next().unwrap();

                    body
                }
                else {
                    String::from("")
                };
    
                let selector = Selector::new(type_char, body)?;
                tokens.push(LexerToken::LiteralSelector(selector));
            },
            '"' => {
                let mut string_body = String::new();

                read_until(&mut data, &mut string_body, |c|!matches!(c,'"'));
                data.next().unwrap();

                tokens.push(LexerToken::LiteralString(string_body));
            }

            _ => return Err(String::from(format!("Unexpected character '{}'", current_char)))
        }
    }

    Ok(tokens)
}

#[derive(Clone)]
#[derive(Debug)]
pub enum LexerToken{
    EndOfFile,
    Whitespace,
    Comment(String),

    Identifier(String),

    FalseKeyword,
    TrueKeyword,
    FloatKeyword,
    IntKeyword,
    BoolKeyword,
    SelectorKeyword,
    StringKeyword,
    FunctionKeyword,

    LiteralInt(i64),
    LiteralFloat(f64),
    LiteralSelector(Selector),
    LiteralString(String),

    OpenParenthesis,
    CloseParenthesis,

    Semicolon,
    Period,
    Plus,
    Dash,
    Star,
    Slash,
    Bang,
    BangEquals,
    Equals,
    DoubleEquals,
}

fn read_until<F>(data: &mut Peekable<Chars>, string: &mut String, is_valid_character: F) where F: Fn(char)->bool {
    loop {
        let last_char = match data.peek() {
            Some(peeked_data) => peeked_data.to_owned(),
            None => break
        };

        if !is_valid_character(last_char) {
            break;
        }
        string.push(last_char);

        data.next().unwrap();
    }
}

fn lex_keyword(keyword: String) -> LexerToken {
    match keyword.as_str()
    {
        "true" => LexerToken::TrueKeyword,
        "false" => LexerToken::FalseKeyword,
        "float" => LexerToken::FloatKeyword,
        "int" => LexerToken::IntKeyword,
        "bool" => LexerToken::BoolKeyword,
        "selector" => LexerToken::SelectorKeyword,
        "string" => LexerToken::StringKeyword,
        "fn" => LexerToken::FunctionKeyword,
        _ => LexerToken::Identifier(keyword)
    }
}