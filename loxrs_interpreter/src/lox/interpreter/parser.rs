use log::{error, trace};

use crate::lox::entities::expr::{
    ExprAssign, ExprBinary, ExprCall, ExprFunction, ExprGet, ExprGrouping, ExprKind, ExprSet,
    ExprUnary,
};
use crate::lox::entities::stmt::{
    StmtBlock, StmtClass, StmtExpr, StmtFun, StmtIf, StmtPrint, StmtReturn, StmtVar, StmtWhile,
};
use crate::lox::entities::{Expr, Literal, Stmt, Token, TokenType, Value};

use loxrs_types::{LoxErr, Result};

use crate::lox::entities::expr::ExprLogical;

const MAX_ARGS_LEN: usize = 255;

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

        Err(self.error(self.peek(), err_message))
    }

    fn error(&self, err_token: &Token, message: &str) -> LoxErr {
        LoxErr::Parse {
            token: message.to_string(),
            line: err_token.line.to_string(),
            column: err_token.column.to_string(),
        }
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

    fn matches(&mut self, token_types: &[TokenType]) -> Option<&Token> {
        for token_type in token_types {
            if self.check(token_type) {
                let res = self.advance();
                return Some(res);
            }
        }
        None
    }

    /// production rules
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
        // declarations
        if self.matches(&[TokenType::Class]).is_some() && self.check(&TokenType::Identifier) {
            stmt = self.class_stmt();
        } else if self.matches(&[TokenType::Fun]).is_some() && self.check(&TokenType::Identifier) {
            stmt = self.fun_stmt("function");
        } else if self.matches(&[TokenType::Var]).is_some() {
            stmt = self.var_stmt();
        }
        // statements
        else if self.matches(&[TokenType::For]).is_some() {
            stmt = self.for_stmt();
        } else if self.matches(&[TokenType::If]).is_some() {
            stmt = self.if_stmt();
        } else if self.matches(&[TokenType::Print]).is_some() {
            stmt = self.print_stmt();
        } else if self.matches(&[TokenType::Return]).is_some() {
            stmt = self.return_stmt();
        } else if self.matches(&[TokenType::While]).is_some() {
            stmt = self.while_stmt();
        } else if self.matches(&[TokenType::LeftBrace]).is_some() {
            stmt = self.block_stmt();
        } else {
            stmt = self.expr_stmt();
        }

        match stmt {
            Ok(val) => Some(val),
            Err(err) => {
                error!("{}", err);
                self.synchronize();
                None
            }
        }
    }

    fn class_stmt(&mut self) -> Result<Stmt> {
        let name = self
            .consume(
                &TokenType::Identifier,
                "Expected `identifier` after `class` keyword.",
            )?
            .to_owned();

        let mut superclass = None;

        if self.matches(&[TokenType::Less]).is_some() {
            superclass = Some(
                self.consume(
                    &TokenType::Identifier,
                    "Expected identifier after superclass `<` character.",
                )?
                .clone(),
            );
        }

        self.consume(&TokenType::LeftBrace, "Expected `{{` before `class` body")?;

        let mut methods: Vec<StmtFun> = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Ok(Stmt::Fun(s)) = self.fun_stmt("method") {
                methods.push(s)
            } else {
                return Err(self.error(self.peek(), "Error in `class` statement body"));
            }
        }

        self.consume(&TokenType::RightBrace, "Expected `}` after class body.")?;
        Ok(Stmt::Class(StmtClass {
            name,
            methods,
            superclass: superclass.map(|e| Expr::new(ExprKind::Var(e))),
        }))
    }

    fn while_stmt(&mut self) -> Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expected `(` after `while` keyword.")?;
        let expr = self.expression()?;
        self.consume(
            &TokenType::RightParen,
            "Expected `)` after `while` condition.",
        )?;

        if let Some(stmt) = self.stmt() {
            Ok(Stmt::While(StmtWhile {
                stmt: Box::new(stmt),
                expr,
            }))
        } else {
            Err(self.error(self.peek(), "Error in `while` statement body"))
        }
    }

    /// Desugars a `for` into `StmtBlock` and `StmtWhile` statements.
    fn for_stmt(&mut self) -> Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expected `(` after `for` statement.")?;

        // init block
        let init;
        if self.matches(&[TokenType::SemiColon]).is_some() {
            init = None;
        } else if self.matches(&[TokenType::Var]).is_some() {
            init = Some(self.var_stmt()?);
        } else {
            init = Some(self.expr_stmt()?);
        }

        // cond block
        let mut cond = None;
        if !self.check(&TokenType::SemiColon) {
            cond = Some(self.expression()?);
        }
        self.consume(&TokenType::SemiColon, "Expected `;` after `for` condition.")?;

        // incr block
        let mut incr = None;
        if !self.check(&TokenType::RightParen) {
            incr = Some(self.expression()?);
        }
        self.consume(&TokenType::RightParen, "Expected `)` after `for` clause.")?;

        let mut body = self
            .stmt()
            .ok_or(self.error(self.peek(), "Error in `for` statement clause"))?;

        if incr.is_some() {
            body = Stmt::Block(StmtBlock {
                stmts: vec![
                    body,
                    Stmt::Expr(StmtExpr {
                        expr: incr.unwrap(),
                    }),
                ],
            });
        }

        if cond.is_none() {
            cond = Some(Expr::new(ExprKind::Literal(Box::new(Literal::Boolean(
                false,
            )))));
        }
        body = Stmt::While(StmtWhile {
            expr: cond.unwrap(),
            stmt: Box::new(body),
        });

        if init.is_some() {
            body = Stmt::Block(StmtBlock {
                stmts: vec![init.unwrap(), body],
            });
        }
        Ok(body)
    }

    fn if_stmt(&mut self) -> Result<Stmt> {
        self.consume(&TokenType::LeftParen, "Expected `(` after `if` statement.")?;
        let cond = self.expression()?;
        self.consume(&TokenType::RightParen, "Expected `)` after `if` condition.")?;

        let then = self.stmt();
        if then.is_none() {
            return Err(self.error(self.peek(), "Error in `if` statement clause"));
        }

        let mut else_stmt = None;
        if self.matches(&[TokenType::Else]).is_some() {
            else_stmt = self.stmt();
            if else_stmt.is_none() {
                return Err(self.error(self.peek(), "Error in `if` statement `else` clause"));
            }
        }

        Ok(Stmt::If(StmtIf {
            cond,
            then: Box::new(then.unwrap()),
            else_stmt: else_stmt.map(Box::new),
        }))
    }

    fn print_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expected `;` after value.")?;
        Ok(Stmt::Print(StmtPrint { expr }))
    }

    fn return_stmt(&mut self) -> Result<Stmt> {
        let keyword = self.previous().clone();
        let mut val = Expr::new(ExprKind::Literal(Box::new(Value::Nil)));

        if !self.check(&TokenType::SemiColon) {
            val = self.expression()?;
        }

        self.consume(
            &TokenType::SemiColon,
            "Expected `;` after return statement.",
        )?;
        Ok(Stmt::Return(StmtReturn { keyword, val }))
    }

    fn block_stmt(&mut self) -> Result<Stmt> {
        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(s) = self.stmt() {
                stmts.push(s)
            }
        }

        self.consume(&TokenType::RightBrace, "Expected `}` after block end.")?;
        Ok(Stmt::Block(StmtBlock { stmts }))
    }

    fn expr_stmt(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expected `;` after expression.")?;
        Ok(Stmt::Expr(StmtExpr { expr }))
    }

    fn fun_stmt(&mut self, kind: &str) -> Result<Stmt> {
        let name = self
            .consume(&TokenType::Identifier, &format!("expected {} name", kind))?
            .clone();

        self.consume(
            &TokenType::LeftParen,
            &format!("expected `(` after `{}` identifier", kind),
        )?;

        let mut params = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() >= MAX_ARGS_LEN {
                    // we throw the error here nonetheless
                    let token = self.peek();

                    return Err(LoxErr::Parse {
                        token: format!(
                            "No more than {} params are allowed for {}",
                            MAX_ARGS_LEN, token
                        ),
                        line: token.line.to_string(),
                        column: token.column.to_string(),
                    });
                }
                params.push(
                    self.consume(&TokenType::Identifier, "expected parameter name")?
                        .clone(),
                );

                if self.matches(&[TokenType::Comma]).is_none() {
                    break;
                }
            }
        }
        self.consume(&TokenType::RightParen, "Expected `)` after params list")?;
        let token = self
            .consume(
                &TokenType::LeftBrace,
                &format!("Expected `{{` before {} body", kind),
            )?
            .clone();

        match self.block_stmt() {
            Err(e) => Err(e),
            Ok(Stmt::Block(block)) => Ok(Stmt::Fun(StmtFun {
                name,
                def: ExprFunction {
                    params,
                    body: block,
                },
            })),
            _ => Err(LoxErr::Parse {
                token: format!("invalid statement in {} declaration", kind),
                line: token.line.to_string(),
                column: token.column.to_string(),
            }),
        }
    }

    fn var_stmt(&mut self) -> Result<Stmt> {
        let token = self
            .consume(&TokenType::Identifier, "expected variable name")?
            .clone();
        let mut expr = None;

        if self.matches(&[TokenType::Equal]).is_some() {
            expr = Some(self.expression()?);
        }

        self.consume(
            &TokenType::SemiColon,
            "Expected `;` after variable declaration.",
        )?;
        Ok(Stmt::Var(StmtVar { token, expr }))
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr> {
        let or_expr = self.or()?;

        if let Some(eq_token) = self.matches(&[TokenType::Equal]).cloned() {
            let val = self.assignment()?;

            match or_expr.kind {
                ExprKind::Var(token) => {
                    return Ok(Expr::new(ExprKind::Assign(Box::new(ExprAssign {
                        name: token,
                        expression: val,
                    }))))
                }
                ExprKind::Get(get) => {
                    return Ok(Expr::new(ExprKind::Set(Box::new(ExprSet {
                        name: get.name,
                        target: get.expr,
                        value: val,
                    }))))
                }
                _ => {
                    // explicitly not returning the error, but displaying it
                    error!("{}", self.error(&eq_token, "Invalid assignment target"));
                }
            }
        }

        Ok(or_expr)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut expr = self.and()?;

        while self.matches(&[TokenType::Or]).is_some() {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::new(ExprKind::Logical(Box::new(ExprLogical {
                left: expr,
                right,
                operator,
            })));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut expr = self.equality()?;

        while self.matches(&[TokenType::And]).is_some() {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::new(ExprKind::Logical(Box::new(ExprLogical {
                left: expr,
                right,
                operator,
            })));
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.comparison()?;
        while self
            .matches(&[TokenType::BangEqual, TokenType::EqualEqual])
            .is_some()
        {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::new(ExprKind::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            })));
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
            expr = Expr::new(ExprKind::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            })));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.matches(&[TokenType::Minus, TokenType::Plus]).is_some() {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::new(ExprKind::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            })));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while self.matches(&[TokenType::Slash, TokenType::Star]).is_some() {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::new(ExprKind::Binary(Box::new(ExprBinary {
                left: expr,
                right,
                operator,
            })));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]).is_some() {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::new(ExprKind::Unary(Box::new(ExprUnary {
                right,
                operator,
            }))));
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr> {
        let mut expr = self.primary()?;

        loop {
            if self.matches(&[TokenType::LeftParen]).is_some() {
                expr = self.finish_call(expr)?;
            } else if self.matches(&[TokenType::Dot]).is_some() {
                let name = self.consume(
                    &TokenType::Identifier,
                    "expected identifier name after `.`.",
                )?;
                expr = Expr::new(ExprKind::Get(Box::new(ExprGet {
                    name: name.clone(),
                    expr,
                })))
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut args = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if args.len() >= MAX_ARGS_LEN {
                    // we throw the error here nonetheless
                    let token = self.peek();

                    return Err(LoxErr::Parse {
                        token: format!(
                            "No more than {} args are allowed for {}",
                            MAX_ARGS_LEN, token
                        ),
                        line: token.line.to_string(),
                        column: token.column.to_string(),
                    });
                }
                args.push(self.expression()?);
                trace!("call args so far: {:?}", args);
                if self.matches(&[TokenType::Comma]).is_none() {
                    break;
                }
            }
        }

        let paren = self.consume(&TokenType::RightParen, "Expected ) after args list")?;
        Ok(Expr::new(ExprKind::Call(Box::new(ExprCall {
            callee,
            paren: paren.clone(),
            args,
        }))))
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matches(&[TokenType::False]).is_some() {
            return Ok(Expr::new(ExprKind::Literal(Box::new(Literal::Boolean(
                false,
            )))));
        }

        if self.matches(&[TokenType::True]).is_some() {
            return Ok(Expr::new(ExprKind::Literal(Box::new(Literal::Boolean(
                true,
            )))));
        }

        if self.matches(&[TokenType::Nil]).is_some() {
            return Ok(Expr::new(ExprKind::Literal(Box::new(Literal::Nil))));
        }

        if let Some(token) = self.matches(&[TokenType::This]) {
            return Ok(Expr::new(ExprKind::This(token.clone())));
        }

        // handle string and num literals
        if let Some(token) = self.matches(&[TokenType::String, TokenType::Number]) {
            return Ok(Expr::new(ExprKind::Literal(Box::new(
                match &token.literal {
                    Some(lit) => lit.clone(),
                    None => Literal::Nil,
                },
            ))));
        }

        if let Some(token) = self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::new(ExprKind::Var(token.clone())));
        }

        if self.matches(&[TokenType::LeftParen]).is_some() {
            let inner_expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expected ) after current token")?;
            return Ok(Expr::new(ExprKind::Grouping(Box::new(ExprGrouping {
                expression: inner_expr,
            }))));
        }

        if let Some(token) = self.matches(&[TokenType::Identifier]) {
            return Ok(Expr::new(ExprKind::Var(token.clone())));
        }

        if self.matches(&[TokenType::Fun]).is_some() {
            return self.func();
        }

        Err(self.error(self.peek(), "expected expression"))
    }

    fn func(&mut self) -> Result<Expr> {
        self.consume(&TokenType::LeftParen, "Expected `(` after `fun` keyword.")?;

        let mut params = vec![];
        if !self.check(&TokenType::RightParen) {
            loop {
                if params.len() >= MAX_ARGS_LEN {
                    // we throw the error here nonetheless
                    let token = self.peek();

                    return Err(LoxErr::Parse {
                        token: format!(
                            "No more than {} params are allowed for {}",
                            MAX_ARGS_LEN, token
                        ),
                        line: token.line.to_string(),
                        column: token.column.to_string(),
                    });
                }
                params.push(
                    self.consume(&TokenType::Identifier, "expected parameter name")?
                        .clone(),
                );

                if self.matches(&[TokenType::Comma]).is_none() {
                    break;
                }
            }
        }

        self.consume(&TokenType::RightParen, "Expected `)` after params list")?;
        let token = self
            .consume(&TokenType::LeftBrace, "Expected `{` before func body")?
            .clone();

        match self.block_stmt() {
            Err(e) => Err(e),
            Ok(Stmt::Block(body)) => Ok(Expr::new(ExprKind::Function(Box::new(ExprFunction {
                params,
                body,
            })))),
            _ => Err(LoxErr::Parse {
                token: "invalid statement in function declaration".to_owned(),
                line: token.line.to_string(),
                column: token.column.to_string(),
            }),
        }
    }
}
