use crate::ast::{Expression, Program, Statement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};

#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest = 0,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

fn precedence_of_infix_operator(t: TokenType) -> Precedence {
    match t {
        TokenType::Eq => Precedence::Equals,
        TokenType::NotEq => Precedence::Equals,
        TokenType::LT => Precedence::LessGreater,
        TokenType::GT => Precedence::LessGreater,
        TokenType::Plus => Precedence::Sum,
        TokenType::Minus => Precedence::Sum,
        TokenType::Slash => Precedence::Product,
        TokenType::Asterisk => Precedence::Product,
        _ => Precedence::Lowest,
    }
}

struct Parser<'a> {
    lexer: &'a mut Lexer,

    current_token: Option<Token>,
    peek_token: Option<Token>,

    errors: Vec<String>,
}

fn dummy_identifier() -> Expression {
    Expression::Identifier {
        token: Token {
            literal: "".to_string(),
            t: TokenType::Ident,
        },
        value: "".to_string(),
    }
}

impl<'a> Parser<'a> {
    fn new(lexer: &mut Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: None,
            peek_token: None,
            errors: Vec::new(),
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
        match self.current_token {
            Some(ref token) => match token.t {
                TokenType::Let => self.parse_let_statement(),
                TokenType::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(),
            },
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        // NOTE: clone -> as_ref로 바꾸면 self에 대한 immutable borrowing이 생기고,
        //       그 이후 mutable borrowing이 불가능해짐.
        // 걸리는게 String을 계속 clone하는건데 이거 나중에 &str로 바꿀 수 있지 않을까?
        // -> 찾아보니까 될 것 같다. chars_indices였나 쓰고 인덱스로 get 메소드 쓰면
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
            value: dummy_identifier(),
        })
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let return_token = self.current_token.clone().unwrap();

        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(Statement::Return {
            token: return_token,
            value: dummy_identifier(),
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let statement = Statement::Expression {
            token: self.current_token.clone().unwrap(),
            expression: self.parse_expression(Precedence::Lowest).unwrap(),
        };

        // NOTE: optional semicolon
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        Some(statement)
    }

    fn is_nud(t: TokenType) -> bool {
        use TokenType::*;
        match t {
            Bang | Minus | Ident | Int => true,
            _ => false,
        }
    }

    fn is_led(t: TokenType) -> bool {
        match t {
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Slash
            | TokenType::Asterisk
            | TokenType::Eq
            | TokenType::NotEq
            | TokenType::LT
            | TokenType::GT => true,
            _ => false,
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        if !Self::is_nud(self.current_token.as_ref().unwrap().t) {
            // TODO: noPrefixParseFnError
            return None;
        }

        let mut left_expression = self.parse_nud();

        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            if !Self::is_led(self.peek_token.as_ref().unwrap().t) {
                return left_expression;
            }

            self.next_token();

            left_expression = self.parse_led(left_expression.unwrap());
        }

        left_expression
    }

    fn parse_nud(&mut self) -> Option<Expression> {
        let current_token = self.current_token.clone().unwrap();

        match current_token.t {
            TokenType::Ident => self.parse_identifier(),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Bang | TokenType::Minus => {
                self.next_token();
                if let Some(right) = self.parse_expression(Precedence::Prefix) {
                    Some(Expression::Prefix {
                        token: current_token.clone(),
                        operator: current_token.literal.to_string(),
                        right: Box::new(right),
                    })
                } else {
                    None
                }
            }
            _ => panic!("should be nud"),
        }
    }

    fn parse_led(&mut self, left: Expression) -> Option<Expression> {
        let token = self.current_token.clone().unwrap();
        let operator = token.literal.clone();
        let precedence = self.current_precedence();
        self.next_token();
        Some(Expression::Infix {
            token,
            operator,
            left: Box::new(left),
            right: Box::new(self.parse_expression(precedence).unwrap()),
        })
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier {
            token: self.current_token.clone().unwrap(),
            value: self.current_token.clone().unwrap().literal,
        })
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        if let Ok(value) = self.current_token.clone().unwrap().literal.parse() {
            Some(Expression::IntegerLiteral {
                token: self.current_token.clone().unwrap(),
                value,
            })
        } else {
            self.errors.push(format!(
                "cloud not parse {} as integer",
                self.current_token.clone().unwrap().literal
            ));
            None
        }
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
            self.peek_error(t);
            false
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(format!(
            "expected next Token to be {:?}, got {:?} instead",
            t,
            self.peek_token.as_ref().unwrap().t
        ));
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_of_infix_operator(self.peek_token.as_ref().unwrap().t)
    }

    fn current_precedence(&self) -> Precedence {
        precedence_of_infix_operator(self.current_token.as_ref().unwrap().t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.len() > 0 {
            for ref e in &parser.errors {
                eprintln!("{}", e);
            }
            panic!("parser error");
        }
    }

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
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 3);

        if let Statement::Let { ref name, .. } = program.statements[0] {
            assert_eq!(name.literal, "x");
        } else {
            panic!("expected let statement");
        }

        if let Statement::Let { ref name, .. } = program.statements[1] {
            assert_eq!(name.literal, "y");
        } else {
            panic!("expected let statement");
        }

        if let Statement::Let { ref name, .. } = program.statements[2] {
            assert_eq!(name.literal, "foobar");
        } else {
            panic!("expected let statement");
        }
    }

    #[test]
    fn return_statement() {
        let input = "
return 5;
return 10;
return 993322;
        ";

        let mut lexer = Lexer::new(input.into());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 3);

        for i in 0..3 {
            if let Statement::Return { ref token, .. } = program.statements[i] {
                assert_eq!(token.literal, "return");
            } else {
                panic!("expected return statement");
            }
        }
    }

    #[test]
    fn identifier_expression() {
        let input = "foobar;";

        let mut lexer = Lexer::new(input.into());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        if let Statement::Expression { ref expression, .. } = &program.statements[0] {
            if let Expression::Identifier { ref value, .. } = expression {
                if value != "foobar" {
                    panic!("expected foobar");
                }
            } else {
                panic!("expected identifier");
            }
        } else {
            panic!("expected expression statement");
        }
    }

    #[test]
    fn integer_literal_expression() {
        let input = "55;";

        let mut lexer = Lexer::new(input.into());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        check_parser_errors(&parser);

        assert_eq!(program.statements.len(), 1);

        if let Statement::Expression { ref expression, .. } = &program.statements[0] {
            if let Expression::IntegerLiteral { value, .. } = expression {
                if value != &55 {
                    panic!();
                }
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn prefix_operator() {
        let tests = vec![("!5;", "!", 5), ("-15;", "-", 15)];

        for test in tests.iter() {
            let (ref input, ref expected_operator, expected_value) = test;

            let mut lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse_program().unwrap();
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);

            if let Statement::Expression { ref expression, .. } = program.statements[0] {
                if let Expression::Prefix {
                    ref operator,
                    ref right,
                    ..
                } = expression
                {
                    assert_eq!(operator, expected_operator);
                    if let Expression::IntegerLiteral { value, .. } = **right {
                        assert_eq!(value, *expected_value);
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }
    }

    #[test]
    fn infix_expression() {
        let tests = vec![
            ("5 + 5;", 5, "+", 5),
            ("5 - 5;", 5, "-", 5),
            ("5 * 5;", 5, "*", 5),
            ("5 / 5;", 5, "/", 5),
            ("5 > 5;", 5, ">", 5),
            ("5 < 5;", 5, "<", 5),
            ("5 == 5;", 5, "==", 5),
            ("5 != 5;", 5, "!=", 5),
        ];

        for (ref input, expected_left_operand, ref expected_operator, expected_right_operand) in
            tests
        {
            let mut lexer = Lexer::new(input.to_string());
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse_program().unwrap();
            check_parser_errors(&parser);

            assert_eq!(program.statements.len(), 1);

            if let Statement::Expression { ref expression, .. } = program.statements[0] {
                if let Expression::Infix {
                    ref operator,
                    ref left,
                    ref right,
                    ..
                } = expression
                {
                    assert_eq!(operator, expected_operator);
                    if let Expression::IntegerLiteral { value, .. } = **left {
                        assert_eq!(value, expected_left_operand);
                    } else {
                        panic!();
                    }
                    if let Expression::IntegerLiteral { value, .. } = **right {
                        assert_eq!(value, expected_right_operand);
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            } else {
                panic!();
            }
        }
    }
}
