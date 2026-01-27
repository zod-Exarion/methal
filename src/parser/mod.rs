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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;
    use expr::{Expression, parse_expression};

    fn parse_expr(input: String) -> Expression {
        let tokens = lexer::tokenize(input);
        let mut iter = tokens.into_iter().peekable();
        parse_expression(&mut iter).unwrap()
    }

    #[test]
    fn parses_number() {
        let expr = parse_expr("42".to_string());

        assert_eq!(expr, Expression::Number(42.0));
    }

    #[test]
    fn parses_simple_addition() {
        let expr = parse_expr("1 + 2".to_string());

        assert_eq!(
            expr,
            Expression::Binary {
                lhs: Box::new(Expression::Number(1.0)),
                op: "+".into(),
                rhs: Box::new(Expression::Number(2.0)),
            }
        );
    }

    #[test]
    fn respects_operator_precedence() {
        let expr = parse_expr("1 + 2 * 3".to_string());

        assert_eq!(
            expr,
            Expression::Binary {
                lhs: Box::new(Expression::Number(1.0)),
                op: "+".into(),
                rhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::Number(2.0)),
                    op: "*".into(),
                    rhs: Box::new(Expression::Number(3.0)),
                }),
            }
        );
    }

    #[test]
    fn parses_unary_expression() {
        let expr = parse_expr("-5".to_string());

        assert_eq!(
            expr,
            Expression::Unary {
                op: "-".into(),
                rhs: Box::new(Expression::Number(5.0)),
            }
        );
    }

    #[test]
    fn parses_let_with_expression() {
        let input = String::from("let f -> 6 + 3 * y");
        let tokens = lexer::tokenize(input);

        let ast = run_parser(tokens);

        match &ast[0] {
            Statement::Let { name, expr, value } => {
                assert_eq!(name, "f");
                assert!(value.is_none());

                // spot-check structure
                match expr {
                    Expression::Binary { op, .. } => assert_eq!(op, "+"),
                    _ => panic!("expected binary expression"),
                }
            }
            _ => panic!("expected let statement"),
        }
    }

    #[test]
    fn parses_let_with_assignment() {
        let input = String::from("let f -> x = 10");
        let tokens = lexer::tokenize(input);

        let ast = run_parser(tokens);

        match &ast[0] {
            Statement::Let { value, .. } => {
                assert!(value.is_some());
            }
            _ => panic!("expected let statement"),
        }
    }

    #[test]
    fn parses_nested_parentheses() {
        let expr = parse_expr("((1 + 2) * (3 + 4))".to_string());

        assert_eq!(
            expr,
            Expression::Binary {
                lhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::Number(1.0)),
                    op: "+".into(),
                    rhs: Box::new(Expression::Number(2.0)),
                }),
                op: "*".into(),
                rhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::Number(3.0)),
                    op: "+".into(),
                    rhs: Box::new(Expression::Number(4.0)),
                }),
            }
        );
    }

    #[test]
    fn unary_with_parentheses() {
        let expr = parse_expr("-(1 + 2)".to_string());

        assert_eq!(
            expr,
            Expression::Unary {
                op: "-".into(),
                rhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::Number(1.0)),
                    op: "+".into(),
                    rhs: Box::new(Expression::Number(2.0)),
                }),
            }
        );
    }
}
