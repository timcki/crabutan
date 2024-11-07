const FN_KEYWORD: &str = "fn";
const LET_KEYWORD: &str = "let";

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Illegal,
    Eof,
    // Identifiers + literals
    Ident(String),
    Int(i64),
    // Operators
    Assign,
    Plus,
    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Keywords
    Function,
    Let,
}

struct Lexer<I>
where
    I: Iterator<Item = char>,
{
    chars: std::iter::Peekable<I>,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn new(chars: I) -> Self {
        Self {
            chars: chars.peekable(),
        }
    }
}

impl<I> Iterator for Lexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // ignore all whitespaces
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.chars.next();
        }

        // return one EOF before returning None
        // TODO: think if it would be a good idea to just interpret None
        // as EOF
        let next_char = match self.chars.next() {
            Some(c) => c,
            None => return Some(Token::Eof),
        };

        let token = match next_char {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            // NOTE: if we want to allow more symbols in tokens write custom
            // is_... func for them
            c if c.is_alphabetic() => {
                let mut ident = String::from(c);
                while let Some(next) = self.chars.peek() {
                    if !next.is_alphanumeric() {
                        break;
                    }
                    ident.push(self.chars.next().unwrap());
                }
                match ident.as_str() {
                    FN_KEYWORD => Token::Function,
                    LET_KEYWORD => Token::Let,
                    _ => Token::Ident(ident),
                }
            }
            // TODO: support more complex numbers
            c if c.is_numeric() => {
                let mut int = String::from(c);
                while let Some(n) = self.chars.peek() {
                    if !n.is_numeric() {
                        break;
                    }
                    int.push(self.chars.next().unwrap());
                }
                Token::Int(int.parse::<i64>().unwrap())
            }

            _ => Token::Illegal,
        };

        Some(token)
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Token::Assign, Token::Assign);
    }

    #[test]
    fn test_simple_token() {
        let input = "=+(){},;";

        let lexer = Lexer::new(input.chars());

        let expected = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        for (token, expected) in lexer.zip(expected) {
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_simple_code() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            "#;
        let expected = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::Eof,
        ];

        let lexer = Lexer::new(input.chars());

        for (token, expected) in lexer.zip(expected) {
            assert_eq!(token, expected);
        }
    }
}
