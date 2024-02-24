use std::f64;

use crate::lox::interpreter::error::LoxErr;

use super::{Expr, Token, TokenType};

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
            TokenType::String(String::from(
                &self.source[(self.start + 1)..(self.current - 1)],
            )),
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
                let num_token =
                    Token::new(TokenType::Number(num, num.to_string()), self.line, curr_col);
                self.tokens.push(num_token);
            }
            Err(err) => self.error(&format!("{}", err))};
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

        let curr_str = &self.source[self.start..self.current];

        let make_token = |t: TokenType| Token::new(t, self.line, self.curr_col());

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
                TokenType::String(String::from(str)),
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
            .push(Token::new(token_type, self.line, curr_col));
    }

    fn curr_col(&self) -> i32 {
        (1 + self.start - self.line_start).try_into().unwrap()
    }

    pub fn scan(&mut self) {
        while !self.is_at_end() {
            self.scan_token();
        }
    }

    fn error(&mut self, message: &str) {
        eprintln!("{}", LoxErr::ScanError {
            line: self.line,
            col: self.col,
            message: message.to_string()
        });
        self.has_errors = true;
    }
}

pub fn run_scanner(raw_s: &str) {
    println!("received: {raw_s}");
    let mut scanner = Scanner::new(String::from(raw_s));

    scanner.scan();

    println!("Here are the tokens that we found: {:#?}", &scanner.tokens);
    println!(
        "and here's a pretty-print representation: {0}",
        Expr::Unary {
            right: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Binary {
                    left: Box::new(Expr::Literal {
                        expr_type: TokenType::Number(324.3, 324.3.to_string()),
                    }),
                    right: Box::new(Expr::Literal {
                        expr_type: TokenType::Number(0.3, 0.3.to_string()),
                    }),
                    operator: Token::new(TokenType::Plus, 8, 9)
                })
            }),
            operator: Token::new(TokenType::Bang, 3, 18)
        }
    )
}
