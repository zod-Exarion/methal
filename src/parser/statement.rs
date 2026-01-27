use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::expect_kind;
use crate::parser::expr::Expression;
use crate::parser::expr::parse_expression;
use crate::parser::parsing_error;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let {
        name: String,
        expr: Expression,
        value: Option<Expression>,
    },

    Assign {
        name: String,
        value: Expression,
    },

    Equation {
        lhs: Expression,
        rhs: Expression,
    },

    Illegal,
}

pub fn parse_let_statement(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Statement {
    // let f -> 6x + 3y = 82742; From = its optional
    tokens.next(); //consume 'let', goes to ident 

    let mut name: String = Default::default();
    let mut expr = Expression::Number(0.0);
    let mut value: Option<Expression> = None;

    match expect_kind(TokenKind::Ident, tokens) {
        Ok(token) => {
            if let Token::Ident(s) = token {
                name = s;
            }
        }
        Err(error) => parsing_error(error, tokens),
    }

    if let Err(error) = expect_kind(TokenKind::Arrow, tokens) {
        parsing_error(error, tokens);
    }

    match parse_expression(tokens) {
        Ok(e) => expr = e,
        Err(error) => parsing_error(error, tokens),
    }

    if expect_kind(TokenKind::Assign, tokens).is_ok() {
        match parse_expression(tokens) {
            Ok(e) => value = Some(e),
            Err(error) => parsing_error(error, tokens),
        }
    }

    Statement::Let { name, expr, value }
}
