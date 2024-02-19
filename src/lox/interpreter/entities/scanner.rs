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
        self.current >= self.source.len()
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
            self.error(self.line, self.col, "Unterminated string.");
            return;
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

        match self.source[self.start..self.current].parse() {
            Ok(num) => self.add_token(TokenType::Number(num)),
            Err(_) => self.error(self.line, self.col, "Invalid number provided"),
        };
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        match &self.source[self.start..self.current] {
            "and" => self.add_token(TokenType::And),
            "class" => self.add_token(TokenType::Class),
            "else" => self.add_token(TokenType::Else),
            "false" => self.add_token(TokenType::False),
            "for" => self.add_token(TokenType::For),
            "fun" => self.add_token(TokenType::Fun),
            "if" => self.add_token(TokenType::If),
            "nil" => self.add_token(TokenType::Nil),
            "or" => self.add_token(TokenType::Or),
            "print" => self.add_token(TokenType::Print),
            "return" => self.add_token(TokenType::Return),
            "super" => self.add_token(TokenType::Super),
            "this" => self.add_token(TokenType::This),
            "true" => self.add_token(TokenType::True),
            "var" => self.add_token(TokenType::Var),
            "while" => self.add_token(TokenType::While),
            _ => self.add_token(TokenType::Identifier),
        }
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
                    self.error(self.line, self.col, &format!("unexpected char: {0}", c));
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            self.line,
            (1 + self.start - self.line_start).try_into().unwrap(),
        ));
    }

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        // add EOF token
        self.tokens
            .push(Token::new(TokenType::Eof, self.line, self.col));
    }

    fn error(&mut self, line: i32, col: i32, message: &str) {
        self.report(line, col, "", message);
    }

    fn report(&mut self, line: i32, col: i32, location: &str, message: &str) {
        eprintln!("[line {line}] [col {col}] Error{location}: {message}");
        self.has_errors = true;
    }
}

pub fn run_scanner(raw_s: &str) {
    println!("received: {raw_s}");
    let mut scanner = Scanner::new(String::from(raw_s));
    scanner.scan();

    println!("Here are the tokens that we found: {:#?}", scanner.tokens);
}
