const FN_KEYWORD: &str = "fn";
const LET_KEYWORD: &str = "let";
const TRUE_KEYWORD: &str = "true";
const FALSE_KEYWORD: &str = "false";
const IF_KEYWORD: &str = "if";
const ELSE_KEYWORD: &str = "else";
const RETURN_KEYWORD: &str = "return";

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    //Eof,
    // Identifiers + literals
    Ident(String),
    Int(i64),
    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,

    Eq,
    NotEq,
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
    True,
    False,
    If,
    Else,
    Return,
}

pub struct Lexer<'a> {
    inner: LexerImpl<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            inner: LexerImpl {
                chars: input.chars().peekable(),
            },
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

struct LexerImpl<I>
where
    I: Iterator<Item = char>,
{
    chars: std::iter::Peekable<I>,
}

// impl<I> LexerImpl<I>
// where
//     I: Iterator<Item = char>,
// {
//     pub fn new(chars: I) -> Self {
//         Self {
//             chars: chars.peekable(),
//         }
//     }
// }

impl<I> Iterator for LexerImpl<I>
where
    I: Iterator<Item = char>,
{
    type Item = Token;

    // next returns `None` as `EOF`
    fn next(&mut self) -> Option<Self::Item> {
        // ignore all whitespaces
        while let Some(&c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.chars.next();
        }

        let token = match self.chars.next()? {
            // TODO: maybe edge case if next char is EOF?
            '=' => match self.chars.peek().copied() {
                Some('=') => {
                    self.chars.next();
                    Token::Eq
                }
                _ => Token::Assign,
            },
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '-' => Token::Minus,
            '!' => match self.chars.peek().copied() {
                Some('=') => {
                    self.chars.next();
                    Token::NotEq
                }
                _ => Token::Bang,
            },
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::Lt,
            '>' => Token::Gt,
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
                    TRUE_KEYWORD => Token::True,
                    FALSE_KEYWORD => Token::False,
                    IF_KEYWORD => Token::If,
                    ELSE_KEYWORD => Token::Else,
                    RETURN_KEYWORD => Token::Return,
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

        let lexer = Lexer::new(input);

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
            //Token::Eof,
        ];

        let lexer = Lexer::new(input);

        for (token, expected) in lexer.zip(expected) {
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_more_symbols() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            //Token::Eof,
        ];

        let lexer = Lexer::new(input);

        for (token, expected) in lexer.zip(expected) {
            //println!("{:?} {:?}", token, expected);
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_even_more_symbols() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }
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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            //Token::Eof,
        ];

        let lexer = Lexer::new(input);

        for (token, expected) in lexer.zip(expected) {
            //println!("{:?} {:?}", token, expected);
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn test_even_even_more_symbols() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
            x + y;
            };
            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
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
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(5),
            Token::Semicolon,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Gt,
            Token::Int(5),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::NotEq,
            Token::Int(9),
            Token::Semicolon,
            //Token::Eof,
        ];

        let lexer = Lexer::new(input);

        for (token, expected) in lexer.zip(expected) {
            //println!("{:?} {:?}", token, expected);
            assert_eq!(token, expected);
        }
    }
}
