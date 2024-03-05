use super::entities::expr::{ExprBinary, ExprGrouping, ExprUnary};
use super::entities::{Expr, Literal, Stmt, Token, TokenType};
use super::error::{LoxErr, Result};

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
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

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() && &self.peek().token_type == token_type
    }

    fn consume(&mut self, token_type: &TokenType, err_message: &str) -> Result<&Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        self.error(self.peek(), err_message);
        // TODO optionally unwind here
        panic!();
    }

    fn error(&self, err_token: &Token, message: &str) -> LoxErr {
        let err = LoxErr::Parse {
            token: err_token.clone(),
            message: message.to_string(),
        };
        eprintln!("{}", err);
        err
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
        todo!();
    }

    fn matches(&mut self, token_types: &[TokenType]) -> Option<&Token> {
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

    pub fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut statements: Vec<Stmt> = vec![];
        while !self.is_at_end() {
            if let Some(stmt) = self.stmt() {
                statements.push(stmt);
            }
        }
        Ok(statements)
    }

    fn stmt(&mut self) -> Option<Stmt> {
        let stmt;
        if self.matches(&[TokenType::Var]).is_some() {
            stmt = self.var_stmt();
        } else if self.matches(&[TokenType::Print]).is_some() {
            stmt = self.print_stmt();
        } else {
            stmt = self.expr_stmt();
        }

        match stmt {
            Ok(val) => Some(val),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn print_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expected `;` after value.")?;
        Ok(Stmt::Print(expr))
    }

    fn expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expected `;` after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn var_stmt(&mut self) -> Result<Stmt> {
        let token = self
            .consume(&TokenType::Identifier, "expected variable name")?
            .clone();
        let mut initializer = None;

        if self.matches(&[TokenType::Equal]).is_some() {
            initializer = Some(self.expression()?);
        }

        self.consume(
            &TokenType::SemiColon,
            "Expected `;` after variable declaration.",
        )?;
        Ok(Stmt::Var(token, initializer))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self
            .matches(&[TokenType::BangEqual, TokenType::EqualEqual])
            .is_some()
        {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            }));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut expr = self.term()?;
        while self
            .matches(&[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ])
            .is_some()
        {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            }));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.matches(&[TokenType::Minus, TokenType::Plus]).is_some() {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            }));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.matches(&[TokenType::Slash, TokenType::Star]).is_some() {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            }));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]).is_some() {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Box::new(ExprUnary { right, operator })));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matches(&[TokenType::False]).is_some() {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }

        if self.matches(&[TokenType::True]).is_some() {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }

        if self.matches(&[TokenType::Nil]).is_some() {
            return Ok(Expr::Literal(Literal::Nil));
        }

        // handle string and num literals
        if let Some(token) = self.matches(&[TokenType::String, TokenType::Number]) {
            return Ok(Expr::Literal(match &token.literal {
                Some(lit) => lit.clone(),
                None => Literal::Nil,
            }));
        }

        if let Some(token) = self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Var(token.clone()));
        }

        if self.matches(&[TokenType::LeftParen]).is_some() {
            let inner_expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ) after this token")?;
            return Ok(Expr::Grouping(Box::new(ExprGrouping {
                expression: inner_expr,
            })));
        }

        if let Some(token) = self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::Var(token.clone()));
        }

        Err(self.error(self.peek(), "expected expression"))
    }
}
