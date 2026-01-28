pub mod evaluator;
pub mod lexer;
pub mod parser;

pub fn run(content: String) {
    let tokens = lexer::tokenize(content);
    let mut parser = parser::Parser::new(tokens.into_iter());
    let ast = parser.run_parser();

    println!("{:?}", ast);
}
