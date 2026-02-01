use crate::parser::{Expression, Operator};

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mult(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Rem(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),

    Abs(Box<Expression>),
    Negative(Box<Expression>),

    Number(f32),
}

pub fn derive_operation(expr: Expression) -> Operation {
    match expr {
        Expression::Number(n) => Operation::Number(n),

        Expression::Binary { op, lhs, rhs } => match op {
            Operator::Add => Operation::Add(lhs, rhs),
            Operator::Sub => Operation::Sub(lhs, rhs),
            Operator::Mult => Operation::Mult(lhs, rhs),
            Operator::Div => Operation::Div(lhs, rhs),
            Operator::Rem => Operation::Rem(lhs, rhs),
            Operator::Pow => Operation::Pow(lhs, rhs),
        },

        Expression::Unary { op, rhs } => match op.as_str() {
            "-" => Operation::Negative(rhs),
            "modulus" => Operation::Abs(rhs),
            _ => unreachable!("unknown unary operator"),
        },

        _ => unreachable!("unknown expression"),
    }
}
