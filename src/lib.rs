pub mod lexer;
pub mod parser;

pub fn run(content: String) {
    let tokens = lexer::tokenize(content);
    let ast = parser::run_parser(tokens);
    println!("{:?}", ast);
}
