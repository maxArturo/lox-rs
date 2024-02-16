use std::char;

use super::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    chars: Vec<char>,
    has_errors: bool,
    start: usize,
    current: usize,
    line: i32,
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
            col: 1,
            has_errors: false,
            tokens: Vec::new(),
        }
    }

    fn advance(&mut self) -> char {
        let res = self.chars[self.current];
        self.current += 1;
        self.col += 1;
        res
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.chars[self.current]
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
        self.current >= self.source.len()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
        }

        // account for closing `"`
        self.advance();

        // create string literal
        self.add_token(TokenType::String(
            self.chars[(self.start + 1)..(self.current - 1)]
                .iter()
                .collect(),
        ));
    }

    fn is_digit(n: char) -> bool {
        n >= '0' && n <= '9'
    }

    fn scan_token(&mut self) {
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
                    self.add_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }

            // comments
            '/' => {
                if self.match_char('/') {
                    // slurp until end of line
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }

            // strings
            '"' => self.string(),

            // whitespaces to ignore
            '\t' => {}
            ' ' => {}

            // handle newlines
            '\n' => {
                self.line += 1;
                self.col = 1;
            }

            // we're also sticking digits and other catchalls here
            _ => {
                // wip
                if self.is_digit(c) {
                    self.number();
                } else {

                    self.error(self.line, &format!("unexpected char: {0}", c)),
                }
            } 
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.chars[self.start..self.current].iter().collect(),
            self.line,
            Some(self.col),
        ));
    }

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // add EOF token
        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            self.line,
            Some(self.col),
        ));
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: i32, location: &str, message: &str) {
        eprintln!("[line {line}] Error{location}: {message}");
        self.has_errors = true;
    }
}

pub fn run_scanner(raw_s: &str) {
    println!("received: {raw_s}");
    let mut scanner = Scanner::new(String::from(raw_s));
    scanner.scan();

    println!("Here are the tokens that we found: {:#?}", scanner.tokens);
}
