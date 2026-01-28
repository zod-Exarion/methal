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
            evaluate(left) + evaluate(right)
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

        _ => unreachable!(),
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

    #[test]
    fn operates_simple() {
        let input = Operation::Add(
            Box::new(Expression::Number(2.0)),
            Box::new(Expression::Number(3.0)),
        );

        let result = evaluate(input);

        assert_eq!(result, 6.0);
    }
}
