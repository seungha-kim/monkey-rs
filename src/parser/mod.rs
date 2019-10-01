use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

struct Parser<'a> {
    lexer: &'a mut Lexer,

    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &mut Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = Some(self.lexer.next_token().clone());
    }

    fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token.is_some()
            && self.current_token.clone().unwrap().t != TokenType::EOF
        {
            let statement = self.parse_statement();
            if let Some(statement) = statement {
                program.statements.push(statement);
            }
            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(ref token) = self.current_token {
            match token.t {
                TokenType::Let => self.parse_let_statement(),
                _ => None,
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        // NOTE: clone -> as_ref로 바꾸면 self에 대한 immutable borrowing이 생기고,
        //       그 이후 mutable borrowing이 불가능해짐.
        // 걸리는게 String을 계속 clone하는건데 이거 나중에 &str로 바꿀 수 있지 않을까?
        let let_token = self.current_token.clone().unwrap();

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name_token = self.current_token.clone().unwrap();

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        let assign_token = self.current_token.clone().unwrap();

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Let {
            name: name_token,
            token: let_token,
            //            value: Box::new(int_token),
        })
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        return self.current_token.is_some() && self.current_token.clone().unwrap().t == t;
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        return self.peek_token.is_some() && self.peek_token.clone().unwrap().t == t;
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[test]
    fn let_statement() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
        ";

        let mut lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 3);

        if let Statement::Let { token: _, ref name } = program.statements[0] {
            assert_eq!(name.literal, "x");
        } else {
            panic!("expected let statement");
        }

        if let Statement::Let { token: _, ref name } = program.statements[1] {
            assert_eq!(name.literal, "y");
        } else {
            panic!("expected let statement");
        }

        if let Statement::Let { token: _, ref name } = program.statements[2] {
            assert_eq!(name.literal, "foobar");
        } else {
            panic!("expected let statement");
        }
    }

}
