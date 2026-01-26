#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Pow,

    LParen,
    RParen,

    Number(f32),
    Ident(String),

    Let,
    Assign,
    Arrow,

    Illegal(char),
}

pub fn read_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    let mut num = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() || c == '.' {
            num.push(c);
            chars.next();
        } else {
            break;
        }
    }

    Token::Number(num.parse::<f32>().unwrap())
}

pub fn read_string(chars: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    let mut ident = String::new();

    while let Some(&c) = chars.peek() {
        if c.is_ascii_alphabetic() {
            ident.push(c);
            chars.next();
        } else {
            break;
        }
    }

    match ident.as_str() {
        "let" => Token::Let,
        _ => Token::Ident(ident),
    }
}
