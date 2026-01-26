pub mod lexer;

pub fn run(content: String) {
    let tokens = lexer::tokenize(content);
    println!("{:?}", tokens);
}
