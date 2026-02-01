use crate::lexer::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    Div,
    Rem,
    Pow,
}

pub fn get_operator(op_token: &Token) -> Operator {
    match op_token {
        Token::Plus => Operator::Add,
        Token::Minus => Operator::Sub,
        Token::Mult => Operator::Mult,
        Token::Div => Operator::Div,
        Token::Mod => Operator::Rem,
        Token::Pow => Operator::Pow,

        _ => unreachable!(),
    }
}
