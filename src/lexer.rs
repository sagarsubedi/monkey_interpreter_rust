use crate::token::{lookup_ident, Token, TokenKind};

struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: Default::default(),
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position]
        }
        println!(
            "Reading char: '{}' at position {}",
            self.ch, self.read_position
        );
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn is_letter(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        ch.is_numeric()
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while Lexer::is_letter(self.ch) {
            identifier.push(self.ch);
            self.read_char();
        }
        identifier
    }

    fn read_number(&mut self) -> String {
        let mut num = String::from("");
        while Lexer::is_digit(self.ch) {
            num.push(self.ch);
            self.read_char();
        }
        num
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.ch {
            ';' => {
                let tok = Lexer::new_token(TokenKind::Semicolon, self.ch);
                self.read_char();
                tok
            }
            '=' => {
                let tok = Lexer::new_token(TokenKind::Assign, self.ch);
                self.read_char();
                tok
            }
            '(' => {
                let tok = Lexer::new_token(TokenKind::Lparen, self.ch);
                self.read_char();
                tok
            }
            ')' => {
                let tok = Lexer::new_token(TokenKind::Rparen, self.ch);
                self.read_char();
                tok
            }
            ',' => {
                let tok = Lexer::new_token(TokenKind::Comma, self.ch);
                self.read_char();
                tok
            }
            '+' => {
                let tok = Lexer::new_token(TokenKind::Plus, self.ch);
                self.read_char();
                tok
            }
            '{' => {
                let tok = Lexer::new_token(TokenKind::Lbrace, self.ch);
                self.read_char();
                tok
            }
            '}' => {
                let tok = Lexer::new_token(TokenKind::Rbrace, self.ch);
                self.read_char();
                tok
            }
            '-' => {
                let tok = Lexer::new_token(TokenKind::Minus, self.ch);
                self.read_char();
                tok
            }
            '*' => {
                let tok = Lexer::new_token(TokenKind::Asterisk, self.ch);
                self.read_char();
                tok
            }
            '/' => {
                let tok = Lexer::new_token(TokenKind::Slash, self.ch);
                self.read_char();
                tok
            }
            '<' => {
                let tok = Lexer::new_token(TokenKind::Lt, self.ch);
                self.read_char();
                tok
            }
            '>' => {
                let tok = Lexer::new_token(TokenKind::Gt, self.ch);
                self.read_char();
                tok
            }
            '!' => {
                let tok = Lexer::new_token(TokenKind::Bang, self.ch);
                self.read_char();
                tok
            }
            '\0' => Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
            _ => {
                if Lexer::is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let kind = lookup_ident(&literal);
                    Token { kind, literal }
                } else if Lexer::is_digit(self.ch) {
                    let literal = self.read_number();
                    let kind = TokenKind::Int;
                    Token { kind, literal }
                } else {
                    let tok = Lexer::new_token(TokenKind::Illegal, self.ch);
                    self.read_char();
                    tok
                }
            }
        };
        token
    }

    fn new_token(kind: TokenKind, ch: char) -> Token {
        Token {
            kind,
            literal: ch.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::token::{Token, TokenKind};

    #[test]
    fn test_next_token() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y){
                x+y;
            };
            let result = add(five, ten);
            *+-/<>!;
            "#;
        let expected: Vec<Token> = vec![
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "5".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Int,
                literal: "10".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Function,
                literal: "fn".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "x".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "y".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Let,
                literal: "let".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "result".to_string(),
            },
            Token {
                kind: TokenKind::Assign,
                literal: "=".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "add".to_string(),
            },
            Token {
                kind: TokenKind::Lparen,
                literal: "(".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "five".to_string(),
            },
            Token {
                kind: TokenKind::Comma,
                literal: ",".to_string(),
            },
            Token {
                kind: TokenKind::Ident,
                literal: "ten".to_string(),
            },
            Token {
                kind: TokenKind::Rparen,
                literal: ")".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                kind: TokenKind::Plus,
                literal: "+".to_string(),
            },
            Token {
                kind: TokenKind::Minus,
                literal: "-".to_string(),
            },
            Token {
                kind: TokenKind::Slash,
                literal: "/".to_string(),
            },
            Token {
                kind: TokenKind::Lt,
                literal: "<".to_string(),
            },
            Token {
                kind: TokenKind::Gt,
                literal: ">".to_string(),
            },
            Token {
                kind: TokenKind::Bang,
                literal: "!".to_string(),
            },
            Token {
                kind: TokenKind::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                kind: TokenKind::Eof,
                literal: "".to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
        for (idx, exp_token) in expected.into_iter().enumerate() {
            let recv_token = lexer.next_token();
            assert_eq!(
                exp_token.literal, recv_token.literal,
                "tests[{idx}] - literal wrong. expected={}, got={}",
                exp_token.literal, recv_token.literal
            );
            assert_eq!(
                exp_token.kind, recv_token.kind,
                "tests[{idx}] - token type wrong. expected={}, got={}",
                exp_token.kind, recv_token.kind
            );
        }
    }
}
