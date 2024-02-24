use super::entities::{Expr, Token, TokenType};
use super::error::{LoxErr, Result};

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>, current: usize) -> Self {
        Self { tokens, current }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn check(&self, token_type: TokenType) -> bool {
        !self.is_at_end() && self.peek().token_type == token_type
    }

    fn consume(&mut self, token_type: TokenType, err_message: &str) -> Result<()> {
        if self.check(token_type) {
            self.advance();
            return Ok(());
        }
        self.error(self.peek(), err_message)
    }

    fn error(&self, err_token: &Token, message: &str) -> Result<()> {
        let err = LoxErr::ParseError {
            token: err_token.clone(),
            message: message.to_string(),
        };
        eprintln!("{}", err);
        Err(err)
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }

            if matches!(
                self.peek().token_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }
            self.advance();
        }
    }

    fn match_fn<F>(&mut self, f: F) -> Option<&Token>
    where
        F: Fn(&Token) -> bool,
    {
        if f(self.peek()) {
            let res = self.advance();
            return Some(res);
        }
        None
    }

    fn match_types(&mut self, token_types: Vec<TokenType>) -> Option<&Token> {
        for token_type in token_types {
            if self.check(token_type) {
                let res = self.advance();
                return Some(res);
            }
        }
        None
    }

    /*
     * production rules
     */
    fn expression_rule(&mut self) -> Expr {
        self.equality_rule()
    }

    fn equality_rule(&mut self) -> Expr {
        let mut expr = self.comparison_rule();
        while self
            .match_types(vec![TokenType::BangEqual, TokenType::EqualEqual])
            .is_some()
        {
            let operator = self.previous().clone();
            let right = self.comparison_rule();
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }
        expr
    }

    fn comparison_rule(&mut self) -> Expr {
        let mut expr = self.term_rule();
        while self
            .match_types(vec![
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ])
            .is_some()
        {
            let operator = self.previous().clone();
            let right = self.term_rule();
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }
        expr
    }

    fn term_rule(&mut self) -> Expr {
        let mut expr = self.factor_rule();
        while let Some(_) = self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor_rule();
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }
        expr
    }

    fn factor_rule(&mut self) -> Expr {
        let mut expr = self.unary_rule();
        while let Some(_) = self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary_rule();
            expr = Expr::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }
        expr
    }

    fn unary_rule(&mut self) -> Expr {
        if let Some(_) = self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary_rule();
            return Expr::Unary {
                right: Box::new(right),
                operator,
            };
        }
        self.primary_rule()
    }

    fn primary_rule(&mut self) -> Expr {
        if let Some(token) =
            self.match_types(vec![TokenType::False, TokenType::True, TokenType::Nil])
        {
            return Expr::Literal {
                expr_type: token.token_type.clone(),
            };
        }

        // handle string and num literals
        if let Some(token) = self.match_fn(|t: &Token| {
            matches!(t.token_type, TokenType::Number(_, _) | TokenType::String(_))
        }) {
            return Expr::Literal {
                expr_type: token.token_type.clone(),
            };
        }

        if self.match_types(vec![TokenType::LeftParen]).is_some() {
            let inner_expr = self.expression_rule();
            self.consume(TokenType::RightParen, "Expected ) after this token");
            // TODO here we need to unwind in case there's an error
            return Expr::Grouping {
                expression: Box::new(inner_expr),
            };
        }
        self.error(self.peek(), "expected expression");
        // TODO optionally unwind here
        panic!();
    }
}
