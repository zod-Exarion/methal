pub mod token;
use token::Token;

pub fn tokenize(content: String) -> Vec<Token> {
    let mut token_vec = Vec::new();

    for line in content.lines() {
        let mut chars = line.chars().peekable();

        while let Some(&c) = chars.peek() {
            let token = match c {
                '0'..='9' => token::read_number(&mut chars),
                'a'..='z' | 'A'..='Z' => token::read_string(&mut chars),

                '+' => {
                    chars.next();
                    Token::Plus
                }
                '-' => {
                    let mut token = Token::Minus;
                    chars.next();

                    if let Some(&c) = chars.peek()
                        && c == '>'
                    {
                        chars.next();
                        token = Token::Arrow;
                    }

                    token
                }
                '*' => {
                    chars.next();
                    Token::Mult
                }
                '/' => {
                    chars.next();
                    Token::Div
                }
                '%' => {
                    chars.next();
                    Token::Mod
                }
                '^' => {
                    chars.next();
                    Token::Pow
                }

                '(' => {
                    chars.next();
                    Token::LParen
                }
                ')' => {
                    chars.next();
                    Token::RParen
                }

                '=' => {
                    chars.next();
                    Token::Assign
                }

                ';' => {
                    chars.next();
                    Token::Semicolon
                }

                // Skip whitespace
                ' ' | '\t' | '\n' => {
                    chars.next();
                    continue;
                }

                _ => {
                    chars.next();
                    Token::Illegal(c)
                }
            };

            token_vec.push(token);
        }
    }

    token_vec
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
            Token::Number(5.0),
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Number(7.0),
            Token::Ident("y".to_string()),
            Token::Assign,
            Token::Number(1.0),
        ];

        assert_eq!(tokens, rhs);
    }
}
