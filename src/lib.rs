pub mod evaluator;
pub mod lexer;
pub mod parser;

use crate::evaluator::eval_expr;
use crate::parser::Statement;

pub fn run(content: String) {
    let tokens = lexer::tokenize(content);
    // println!("{:?}", &tokens);
    let mut parser = parser::Parser::new(tokens.into_iter());
    let ast = parser.run_parser();

    for statement in ast.iter() {
        if let Statement::Let {
            name,
            expr: _,
            value,
        } = statement
        {
            println!("let {} = {}", name, eval_expr(value.clone().unwrap()))
        }
    }

    // println!("{:?}", ast);
}
