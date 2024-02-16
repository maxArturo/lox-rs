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
        res
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.chars[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        println!("currently matching:  {:#?}", c);
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

            // whitespaces to ignore
            '\n' => {},
            '\t' => {},
            ' ' => {},
            _ => self.error(self.line, &format!("unexpected char: {0}", c))
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        println!("adding token for {:?}", token_type);
        self.tokens.push(Token::new(
            token_type,
            self.chars[self.start..self.current].iter().collect(),
            self.line,
            Some(self.col),
        ));
        println!("New tokens content: {:#?}", self.tokens);
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
