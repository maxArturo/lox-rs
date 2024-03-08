use std::f64;

use super::entities::{Literal, Stmt, Token, TokenType};
use crate::lox::interpreter::{
    error::{LoxErr, Result},
    parser,
};
use log::debug;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    chars: Vec<char>,
    errors: Option<Vec<LoxErr>>,
    start: usize,
    current: usize,
    line: i32,
    line_start: usize,
    col: i32,
    tokens: Vec<Token>,
}

impl Scanner {
    fn new(source: String) -> Self {
        let chars = source.chars().collect();
        Self {
            source,
            chars,
            start: 0,
            current: 0,
            line: 1,
            line_start: 0,
            col: 1,
            errors: None,
            tokens: Vec::new(),
        }
    }

    fn advance(&mut self) -> char {
        let res = self.chars[self.current];
        self.current += 1;
        res
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.chars[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.chars.len() {
            return '\0';
        }
        self.chars[self.current + 1]
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&self) -> bool {
        let curr = self.current;
        let source_len = self.source.len();
        curr >= source_len
    }

    fn set_next_line(&mut self) {
        self.line += 1;
        self.line_start = self.current;
        self.col = 1;
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.set_next_line();
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("reached end of input");
            return;
        }

        // account for closing `"`
        self.advance();
        let curr_col = self.curr_col();

        let str_token = Token::new(
            TokenType::String,
            Some(Literal::String(String::from(
                &self.source[(self.start + 1)..(self.current - 1)],
            ))),
            self.line,
            curr_col,
        );

        self.tokens.push(str_token);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // look for fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();

            // consume remaining fractions
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let curr_col = self.curr_col();
        match self.source[self.start..self.current].parse::<f64>() {
            Ok(num) => {
                let num_token = Token::new(
                    TokenType::Number,
                    Some(Literal::Number(num)),
                    self.line,
                    curr_col,
                );
                self.tokens.push(num_token);
            }
            Err(err) => self.error(&format!("{}", err)),
        };
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let curr_str = &self.source[self.start..self.current];

        let make_token = |t: TokenType| Token::new(t, None, self.line, self.curr_col());

        let new_token = match curr_str {
            "and" => make_token(TokenType::And),
            "class" => make_token(TokenType::Class),
            "else" => make_token(TokenType::Else),
            "false" => make_token(TokenType::False),
            "for" => make_token(TokenType::For),
            "fun" => make_token(TokenType::Fun),
            "if" => make_token(TokenType::If),
            "nil" => make_token(TokenType::Nil),
            "or" => make_token(TokenType::Or),
            "print" => make_token(TokenType::Print),
            "return" => make_token(TokenType::Return),
            "super" => make_token(TokenType::Super),
            "this" => make_token(TokenType::This),
            "true" => make_token(TokenType::True),
            "var" => make_token(TokenType::Var),
            "while" => make_token(TokenType::While),
            str => Token::new(
                TokenType::Identifier,
                Some(Literal::String(str.to_string())),
                self.line,
                self.curr_col(),
            ),
        };
        self.tokens.push(new_token);
    }

    fn scan_token(&mut self) {
        self.start = self.current;
        let c = self.advance();

        match c {
            // single char literals
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),

            // multichar literals
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }

            // comments
            '/' => {
                if self.match_char('/') {
                    // slurp until end of line
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    // slurp until next end comment tokens
                    while !self.is_at_end() {
                        match self.peek() {
                            '\n' => {
                                self.advance();
                                self.set_next_line();
                            }
                            '*' => {
                                if self.peek_next() == '/' {
                                    self.advance();
                                    self.advance();
                                    break;
                                }
                                self.advance();
                            }
                            _ => {
                                self.advance();
                            }
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            // strings
            '"' => self.string(),

            // whitespaces to ignore
            '\t' | ' ' => {}

            // handle newlines
            '\n' => {
                self.set_next_line();
            }

            // we're also sticking digits and identifiers/catchalls here
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.error(&format!("unexpected char: {0}", c));
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let curr_col = self.curr_col();
        self.tokens
            .push(Token::new(token_type, None, self.line, curr_col));
    }

    fn curr_col(&self) -> i32 {
        (1 + self.start - self.line_start).try_into().unwrap()
    }

    fn error(&mut self, message: &str) {
        let err = || LoxErr::Scan {
            line: self.line,
            col: self.col,
            message: message.to_string(),
        };

        match &mut self.errors {
            Some(errs) => {
                errs.push(err());
            }
            None => {
                self.errors = Some(vec![err()]);
            }
        };
    }
}

pub trait Scan {
    fn new(source: &str) -> Self;
    fn scan(&mut self) -> Result<Vec<Token>, Vec<LoxErr>>;
}

impl Scan for Scanner {
    fn scan(&mut self) -> Result<Vec<Token>, Vec<LoxErr>> {
        while !self.is_at_end() {
            self.scan_token();
        }
        self.add_token(TokenType::Eof);

        match &self.errors {
            Some(errs) => Err(errs.clone()),
            None => Ok(self.tokens.clone()),
        }
    }

    fn new(source: &str) -> Self {
        let chars = source.chars().collect();
        Self {
            source: source.to_string(),
            chars,
            start: 0,
            current: 0,
            line: 1,
            line_start: 0,
            col: 1,
            errors: None,
            tokens: Vec::new(),
        }
    }
}

pub fn scan_parse(raw_s: &str) -> Result<Vec<Stmt>, Vec<LoxErr>> {
    debug!("received: {raw_s}");
    let mut scanner = Scanner::new(String::from(raw_s));

    let tokens = scanner.scan()?;

    debug!("Here are the tokens that we found: {:#?}", &tokens);
    let mut parser = parser::Parser::new(tokens);

    parser.parse().map_err(|e| vec![e])
}
