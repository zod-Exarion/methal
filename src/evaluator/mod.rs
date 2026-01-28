pub mod operation;
use crate::parser::expr::Expression;
use operation::Operation;
use operation::derive_operation;

pub fn evaluate(operation: Operation) -> f32 {
    match operation {
        Operation::Number(n) => n,

        Operation::Add(lhs, rhs) => {
            let (left, right) = get_operation(lhs, rhs);
            evaluate(left) + evaluate(right)
        }
        Operation::Sub(lhs, rhs) => {
            let (left, right) = get_operation(lhs, rhs);
            evaluate(left) - evaluate(right)
        }
        Operation::Mult(lhs, rhs) => {
            let (left, right) = get_operation(lhs, rhs);
            evaluate(left) * evaluate(right)
        }
        Operation::Div(lhs, rhs) => {
            let (left, right) = get_operation(lhs, rhs);
            evaluate(left) / evaluate(right)
        }
        Operation::Rem(lhs, rhs) => {
            let (left, right) = get_operation(lhs, rhs);
            evaluate(left) % evaluate(right)
        }
        Operation::Pow(lhs, rhs) => {
            let (left, right) = get_operation(lhs, rhs);
            evaluate(left).powf(evaluate(right))
        }

        Operation::Negative(expr) => -evaluate(derive_operation(*expr)),

        Operation::Abs(expr) => evaluate(derive_operation(*expr)).abs(),
    }
}

fn get_operation(lhs: Box<Expression>, rhs: Box<Expression>) -> (Operation, Operation) {
    let lhs = derive_operation(*lhs); // uhh * to destructure Box
    let rhs = derive_operation(*rhs);

    (lhs, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expr::Expression;

    fn num(n: f32) -> Box<Expression> {
        Box::new(Expression::Number(n))
    }

    fn eval_expr(expr: Expression) -> f32 {
        evaluate(derive_operation(expr))
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
}
