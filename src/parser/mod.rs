pub mod expr;
pub mod statement;

use crate::lexer::token::Token;
use statement::Statement;
use std::iter::Peekable;

pub fn run_parser(tokens: Vec<Token>) -> Vec<Statement> {
    let mut tokens = tokens.into_iter().peekable();
    let mut statement_vec = Vec::new();

    while let Some(token) = tokens.peek() {
        let statement = match token {
            Token::Let => statement::parse_let_statement(&mut tokens),
            _ => {
                parsing_error(String::from("Illegal statement"), &mut tokens);
                Statement::Illegal // the compiler is nagging me so
            }
        };
        statement_vec.push(statement);
        tokens.next(); // Make sure to parse without consuming last token, consume it here
    }

    statement_vec
}

//Advances TWICE
pub fn expect_kind(
    kind: crate::lexer::token::TokenKind,
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Token, String> {
    match tokens.peek() {
        Some(token) if token.kind() == kind => {
            let tok = tokens.next();
            Ok(tok.unwrap())
        }
        other => Err(format!("expected {:?}, got {:?}", kind, other)),
    }
}

pub fn parsing_error(error: String, tokens: &mut Peekable<impl Iterator<Item = Token>>) {
    eprintln!(
        "{error}: failed_at->{:?}, next->{:?}",
        tokens.next().unwrap(),
        tokens.peek().unwrap()
    );
    std::process::exit(1);
}
