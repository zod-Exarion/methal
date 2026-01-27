use crate::lexer::token::Token;
use std::iter::Peekable;

#[derive(PartialEq, Debug)]
pub enum Expression {
    Number(f32),
    Ident(String),

    Binary {
        lhs: Box<Expression>,
        op: String,
        rhs: Box<Expression>,
    },
    Unary {
        op: String,
        rhs: Box<Expression>,
    },
}

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Sum,     // + -
    Product, // * /
    Prefix,  // -x
    Parens,  // ( )
}

pub fn token_precedence(token: &Token) -> Precedence {
    match token {
        Token::Plus | Token::Minus => Precedence::Sum,
        Token::Mult | Token::Div => Precedence::Product,
        _ => Precedence::Lowest,
    }
}

fn parse_prefix(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Expression, String> {
    match tokens.next() {
        Some(Token::Number(n)) => Ok(Expression::Number(n)),
        Some(Token::Ident(s)) => Ok(Expression::Ident(s)),

        Some(Token::Minus) => {
            let rhs = parse_expression_pratt(tokens, Precedence::Prefix)?;
            Ok(Expression::Unary {
                op: "-".into(),
                rhs: Box::new(rhs),
            })
        }

        Some(Token::LParen) => {
            let expr = parse_expression_pratt(tokens, Precedence::Lowest)?;

            match tokens.next() {
                Some(Token::RParen) => Ok(expr),
                other => Err(format!("expected ')', got {:?}", other)),
            }
        }

        other => Err(format!("unexpected token in expression: {:?}", other)),
    }
}

fn parse_infix(
    lhs: Expression,
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, String> {
    let op_token = tokens.next().unwrap();
    let precedence = token_precedence(&op_token);

    let rhs = parse_expression_pratt(tokens, precedence)?;

    let op = match op_token {
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Mult => "*",
        Token::Div => "/",
        _ => unreachable!(),
    };

    Ok(Expression::Binary {
        lhs: Box::new(lhs),
        op: op.into(),
        rhs: Box::new(rhs),
    })
}

fn parse_expression_pratt(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
    min_prec: Precedence,
) -> Result<Expression, String> {
    let mut lhs = parse_prefix(tokens)?;

    while let Some(next) = tokens.peek() {
        let prec = token_precedence(next);

        if prec <= min_prec {
            break;
        }

        lhs = parse_infix(lhs, tokens)?;
    }

    Ok(lhs)
}

pub fn parse_expression(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Expression, String> {
    parse_expression_pratt(tokens, Precedence::Lowest)
}
