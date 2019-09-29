use super::token::*;

trait Identifier {
    fn is_identifier(&self) -> bool;
}

impl Identifier for char {
    fn is_identifier(&self) -> bool {
        self.is_alphabetic() || self == &'_'
    }
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    current: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            current: None,
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token = match self.current {
            Some(ch @ '=') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token {
                        t: TokenType::Eq,
                        literal: "==".to_string(),
                    }
                } else {
                    Token {
                        t: TokenType::Assign,
                        literal: ch.to_string(),
                    }
                }
            }
            Some(ch @ '+') => Token {
                t: TokenType::Plus,
                literal: ch.to_string(),
            },
            Some(ch @ '-') => Token {
                t: TokenType::Minus,
                literal: ch.to_string(),
            },
            Some(ch @ '!') => {
                if self.peek_char() == Some('=') {
                    self.read_char();
                    Token {
                        t: TokenType::NotEq,
                        literal: "!=".to_string(),
                    }
                } else {
                    Token {
                        t: TokenType::Bang,
                        literal: ch.to_string(),
                    }
                }
            }
            Some(ch @ '/') => Token {
                t: TokenType::Slash,
                literal: ch.to_string(),
            },
            Some(ch @ '*') => Token {
                t: TokenType::Asterisk,
                literal: ch.to_string(),
            },
            Some(ch @ '<') => Token {
                t: TokenType::LT,
                literal: ch.to_string(),
            },
            Some(ch @ '>') => Token {
                t: TokenType::GT,
                literal: ch.to_string(),
            },
            Some(ch @ ';') => Token {
                t: TokenType::Semicolon,
                literal: ch.to_string(),
            },
            Some(ch @ '(') => Token {
                t: TokenType::LeftParen,
                literal: ch.to_string(),
            },
            Some(ch @ ')') => Token {
                t: TokenType::RightParen,
                literal: ch.to_string(),
            },
            Some(ch @ ',') => Token {
                t: TokenType::Comma,
                literal: ch.to_string(),
            },
            Some(ch @ '{') => Token {
                t: TokenType::LeftBrace,
                literal: ch.to_string(),
            },
            Some(ch @ '}') => Token {
                t: TokenType::RightBrace,
                literal: ch.to_string(),
            },
            None => Token {
                t: TokenType::EOF,
                literal: "".to_string(),
            },
            Some(ch) => {
                if ch.is_identifier() {
                    let literal = self.read_identifier();
                    return Token {
                        t: lookup_ident(&literal),
                        literal,
                    };
                } else if ch.is_digit(10) {
                    let literal = self.read_number();
                    return Token {
                        t: TokenType::Int,
                        literal,
                    };
                } else {
                    Token {
                        t: TokenType::Illegal,
                        literal: ch.to_string(),
                    }
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.current = None;
        } else {
            // FIXME
            self.current = self.input.chars().nth(self.read_position);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.current.filter(char::is_identifier).is_some() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.current.filter(|&c| char::is_digit(c, 10)).is_some() {
            self.read_char()
        }
        self.input[position..self.position].to_string()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input.chars().nth(self.read_position)
    }

    fn skip_whitespace(&mut self) {
        while match self.current {
            Some(ch) if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' => true,
            _ => false,
        } {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = r"let five = 5;
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
10 != 9;"
            .to_string();
        let mut lexer = Lexer::new(input);

        let tests = vec![
            Token {
                t: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                t: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                t: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                t: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t: TokenType::Function,
                literal: "fn".to_string(),
            },
            Token {
                t: TokenType::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                t: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "y".to_string(),
            },
            Token {
                t: TokenType::RightParen,
                literal: ")".to_string(),
            },
            Token {
                t: TokenType::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "x".to_string(),
            },
            Token {
                t: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "y".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::Let,
                literal: "let".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "result".to_string(),
            },
            Token {
                t: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "add".to_string(),
            },
            Token {
                t: TokenType::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "five".to_string(),
            },
            Token {
                t: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                t: TokenType::Ident,
                literal: "ten".to_string(),
            },
            Token {
                t: TokenType::RightParen,
                literal: ")".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::Bang,
                literal: "!".to_string(),
            },
            Token {
                t: TokenType::Minus,
                literal: "-".to_string(),
            },
            Token {
                t: TokenType::Slash,
                literal: "/".to_string(),
            },
            Token {
                t: TokenType::Asterisk,
                literal: "*".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t: TokenType::LT,
                literal: "<".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t: TokenType::GT,
                literal: ">".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::If,
                literal: "if".to_string(),
            },
            Token {
                t: TokenType::LeftParen,
                literal: "(".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "5".to_string(),
            },
            Token {
                t: TokenType::LT,
                literal: "<".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t: TokenType::RightParen,
                literal: ")".to_string(),
            },
            Token {
                t: TokenType::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                t: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                t: TokenType::True,
                literal: "true".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                t: TokenType::Else,
                literal: "else".to_string(),
            },
            Token {
                t: TokenType::LeftBrace,
                literal: "{".to_string(),
            },
            Token {
                t: TokenType::Return,
                literal: "return".to_string(),
            },
            Token {
                t: TokenType::False,
                literal: "false".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::RightBrace,
                literal: "}".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t: TokenType::Eq,
                literal: "==".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "10".to_string(),
            },
            Token {
                t: TokenType::NotEq,
                literal: "!=".to_string(),
            },
            Token {
                t: TokenType::Int,
                literal: "9".to_string(),
            },
            Token {
                t: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                t: TokenType::EOF,
                literal: "".to_string(),
            },
        ];

        for test_token in tests {
            let current_token = lexer.next_token();
            assert_eq!(current_token, test_token);
        }
    }
}
