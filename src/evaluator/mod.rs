pub mod operation;
use crate::parser::expr::Expression;
use operation::Operation;
use operation::derive_operation;

pub fn eval_expr(expr: Expression) -> f32 {
    evaluate(derive_operation(expr))
}

pub fn evaluate(operation: Operation) -> f32 {
    match operation {
        Operation::Number(n) => n,

        Operation::Add(lhs, rhs) => eval_expr(*lhs) + eval_expr(*rhs),
        Operation::Sub(lhs, rhs) => eval_expr(*lhs) - eval_expr(*rhs),
        Operation::Mult(lhs, rhs) => eval_expr(*lhs) * eval_expr(*rhs),
        Operation::Div(lhs, rhs) => eval_expr(*lhs) / eval_expr(*rhs),
        Operation::Rem(lhs, rhs) => eval_expr(*lhs) % eval_expr(*rhs),
        Operation::Pow(lhs, rhs) => eval_expr(*lhs).powf(eval_expr(*rhs)),

        Operation::Negative(expr) => -eval_expr(*expr),

        Operation::Abs(expr) => eval_expr(*expr).abs(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expr::Expression;

    fn num(n: f32) -> Box<Expression> {
        Box::new(Expression::Number(n))
    }

    #[test]
    fn test_addition() {
        let expr = Expression::Binary {
            op: "+".into(),
            lhs: num(2.0),
            rhs: num(3.0),
        };
        assert_eq!(eval_expr(expr), 5.0);
    }

    #[test]
    fn test_subtraction() {
        let expr = Expression::Binary {
            op: "-".into(),
            lhs: num(10.0),
            rhs: num(4.0),
        };
        assert_eq!(eval_expr(expr), 6.0);
    }

    #[test]
    fn test_multiplication() {
        let expr = Expression::Binary {
            op: "*".into(),
            lhs: num(6.0),
            rhs: num(7.0),
        };
        assert_eq!(eval_expr(expr), 42.0);
    }

    #[test]
    fn test_division() {
        let expr = Expression::Binary {
            op: "/".into(),
            lhs: num(8.0),
            rhs: num(2.0),
        };
        assert_eq!(eval_expr(expr), 4.0);
    }

    #[test]
    fn test_remainder() {
        let expr = Expression::Binary {
            op: "%".into(),
            lhs: num(10.0),
            rhs: num(3.0),
        };
        assert_eq!(eval_expr(expr), 1.0);
    }

    #[test]
    fn test_power() {
        let expr = Expression::Binary {
            op: "^".into(),
            lhs: num(2.0),
            rhs: num(3.0),
        };
        assert_eq!(eval_expr(expr), 8.0);
    }

    #[test]
    fn test_negative_number() {
        let expr = Expression::Unary {
            op: "-".into(),
            rhs: num(5.0),
        };
        assert_eq!(eval_expr(expr), -5.0);
    }

    #[test]
    fn test_negative_expression() {
        let expr = Expression::Unary {
            op: "-".into(),
            rhs: Box::new(Expression::Binary {
                op: "+".into(),
                lhs: num(2.0),
                rhs: num(3.0),
            }),
        };
        assert_eq!(eval_expr(expr), -5.0);
    }

    #[test]
    fn test_absolute_value() {
        let expr = Expression::Unary {
            op: "modulus".into(),
            rhs: num(-7.0),
        };
        assert_eq!(eval_expr(expr), 7.0);
    }

    #[test]
    fn test_binary_expression() {
        let expr: Expression = Expression::Binary {
            lhs: Box::new(Expression::Number(2.0)),
            op: "+".into(),
            rhs: Box::new(Expression::Binary {
                lhs: Box::new(Expression::Number(3.0)),
                op: "-".into(),
                rhs: Box::new(Expression::Number(10.0)),
            }),
        };

        assert_eq!(eval_expr(expr), -5.0);
    }
}
