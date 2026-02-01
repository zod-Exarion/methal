pub mod expr;
pub mod operator;
pub mod statement;

pub use expr::{Expression, parse_expression};
pub use operator::{Operator, get_operator};
pub use statement::Statement;

use crate::lexer::token::Token;
use std::iter::Peekable;

pub struct Parser<I: Iterator<Item = Token>> {
    tokens: Peekable<I>,
}

impl<I: Iterator<Item = Token>> Parser<I> {
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn run_parser(&mut self) -> Vec<Statement> {
        let mut statement_vec = Vec::new();

        while let Some(token) = self.tokens.peek() {
            let statement = match token {
                Token::Let => statement::parse_let_statement(&mut self.tokens),

                Token::Ident(_) | Token::Number(_) | Token::Minus | Token::Pipe => {
                    statement::parse_assign_statement(&mut self.tokens)
                }
                _ => {
                    parsing_error(String::from("Illegal statement"), &mut self.tokens);
                    unreachable!("Illegal statement should have been handled")
                }
            };
            statement_vec.push(statement);
            self.tokens.next(); // Make sure to parse without consuming last token, consume it here
        }

        statement_vec
    }
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
    use crate::parser;
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
                op: Operator::Add,
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
                op: Operator::Add,
                rhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::Number(2.0)),
                    op: Operator::Mult,
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

        let mut parser = parser::Parser::new(tokens.into_iter());
        let ast = parser.run_parser();

        match &ast[0] {
            Statement::Let { name, expr, value } => {
                assert_eq!(name, "f");
                assert!(value.is_none());

                // spot-check structure
                match expr {
                    Expression::Binary { op, .. } => assert_eq!(op, &Operator::Add),
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

        let mut parser = parser::Parser::new(tokens.into_iter());
        let ast = parser.run_parser();

        match &ast[0] {
            Statement::Let { value, .. } => {
                assert!(value.is_some());
            }
            _ => panic!("expected let statement"),
        }
    }

    #[test]
    fn parser_assign_statement() {
        let input = String::from("f+g = 10+h;");
        let tokens = lexer::tokenize(input);

        let mut parser = parser::Parser::new(tokens.into_iter());
        let ast = parser.run_parser();

        match &ast[0] {
            Statement::Assign { expr, value } => {
                assert_eq!(
                    *expr,
                    Expression::Binary {
                        lhs: Box::new(Expression::Ident("f".into())),
                        op: Operator::Add,
                        rhs: Box::new(Expression::Ident("g".into()))
                    }
                );

                assert_eq!(
                    *value,
                    Expression::Binary {
                        lhs: Box::new(Expression::Number(10.0)),
                        op: Operator::Add,
                        rhs: Box::new(Expression::Ident("h".into()))
                    }
                );
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
                    op: Operator::Add,
                    rhs: Box::new(Expression::Number(2.0)),
                }),
                op: Operator::Mult,
                rhs: Box::new(Expression::Binary {
                    lhs: Box::new(Expression::Number(3.0)),
                    op: Operator::Add,
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
                    op: Operator::Add,
                    rhs: Box::new(Expression::Number(2.0)),
                }),
            }
        );
    }
}
