#[derive(PartialEq, Debug)]
pub enum Token {
    Number(f32),
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
    Pow,

    Ident(String),

    Let,
    Assign,
    Arrow,
}

pub fn get_token(word: &str) -> Token {
    match word {
        "let" => Token::Let,
        "+" => Token::Plus,
        "-" => Token::Minus,
        "*" => Token::Mult,
        "/" => Token::Div,
        "%" => Token::Mod,
        "^" => Token::Pow,
        "=" => Token::Assign,
        "->" => Token::Arrow,

        s => num_or_ident(s),
    }
}

fn num_or_ident(word: &str) -> Token {
    match word.parse::<f32>() {
        Ok(num) => Token::Number(num),
        Err(_) => Token::Ident(word.to_string()),
    }
}
