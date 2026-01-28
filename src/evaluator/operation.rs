use crate::parser::expr::Expression;

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

        Expression::Binary { op, lhs, rhs } => match op.as_str() {
            "+" => Operation::Add(lhs, rhs),
            "-" => Operation::Sub(lhs, rhs),
            "*" => Operation::Mult(lhs, rhs),
            "/" => Operation::Div(lhs, rhs),
            "%" => Operation::Rem(lhs, rhs),
            "^" => Operation::Pow(lhs, rhs),
            _ => unreachable!("unknown binary operator"),
        },

        Expression::Unary { op, rhs } => match op.as_str() {
            "-" => Operation::Negative(rhs),
            "modulus" => Operation::Abs(rhs),
            _ => unreachable!("unknown unary operator"),
        },

        _ => unreachable!("unknown expression"),
    }
}
