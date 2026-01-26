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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test() {
        let input = String::from("let f -> 5x + 7y = 1");
        let tokens = tokenize(input);

        let rhs = vec![
            Token::Let,
            Token::Ident("f".to_string()),
            Token::Arrow,
            Token::Ident("5x".to_string()),
            Token::Plus,
            Token::Ident("7y".to_string()),
            Token::Assign,
            Token::Number(1.0),
        ];

        assert_eq!(tokens, rhs);
    }
}
