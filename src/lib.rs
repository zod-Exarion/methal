pub mod lexer;

pub fn run(content: String) {
    let tokens = lexer::tokenize(content);

    lexer::display_tokens(tokens);
}
