mod token;
use token::Token;

pub fn tokenize(content: String) -> Vec<Token> {
    let mut token_vec = Vec::new();

    for line in content.lines() {
        for word in line.split_whitespace() {
            let token = token::get_token(word);
            token_vec.push(token);
        }
    }

    token_vec
}

pub fn display_tokens(tokens: Vec<Token>) {
    println!("{:?}", tokens);
    // for token in &tokens {
    //     println!("{:?}", token);
    // }
}
