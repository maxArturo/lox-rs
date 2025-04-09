use std::ops::Range;

use log::debug;

use crate::{
    constants::NO_SPAN,
    entities::{chunk::Chunk, opcode, precedence, value::Value},
    error::{CompilerError, InternalError, LoxErrorS, Result},
    parser::Parser,
    scanner::{scan, Token, TokenS},
    types::Span,
};

pub fn compile(source: &str) -> Result<Chunk, Vec<LoxErrorS>> {
    let mut compiler = Compiler::new(source)?;
    compiler.compile()?;
    Ok(compiler.chunk)
}

struct Compiler {
    chunk: Chunk,
    parser: Parser,
}

impl Compiler {
    fn new(source: &str) -> Result<Self, Vec<LoxErrorS>> {
        let parser = Parser::new(scan(source)?);
        Ok(Self {
            chunk: Chunk::new(),
            parser,
        })
    }

    fn compile(&mut self) -> Result<&Chunk, Vec<LoxErrorS>> {
        // temporarily add a return opcode
        if let Err(e) = self.expression() {
            return Err(vec![e]);
        }

        match self.emit_return() {
            Ok(_) => Ok(&self.chunk),
            Err(e) => return Err(vec![e]),
        }
    }

    fn expression(&mut self) -> Result<(), LoxErrorS> {
        self.parse_precedence(precedence::PREC_ASSIGNMENT)
    }

    fn grouping(&mut self) -> Result<(), LoxErrorS> {
        self.expression()?;
        self.parser
            .consume(Token::RightParen, "Expected `)` after expression")
    }

    fn parse_precedence(&mut self, prec: u8) -> Result<(), LoxErrorS> {
        self.parser.advance();

        let prefix_rule = {
            let prev = self.parser.prev.as_ref();
            match prev {
                None => return Ok(()),
                Some((Token::EndOfFile, _)) => return Ok(()),
                Some(token_s) => {
                    debug!("[COMPILER] getting `prefix_rule` for: {:?}", prev);
                    Compiler::get_rule(token_s)?.prefix.ok_or_else(|| {
                        (
                            CompilerError::PrecedenceError(token_s.0.to_string()).into(),
                            token_s.1.clone(),
                        )
                    })?
                }
            }
        };

        prefix_rule(self)?;

        loop {
            let should_break = {
                let curr_token = self.parser.curr.as_ref();
                match curr_token {
                    None => return Ok(()),
                    Some((Token::EndOfFile, _)) => return Ok(()),
                    Some(token_s) => prec > Compiler::get_rule(token_s)?.precedence,
                }
            };

            if should_break {
                break;
            }

            self.parser.advance();

            let infix_rule = {
                let prev = self.parser.prev.as_ref();

                match prev {
                    None => return Ok(()),
                    Some((Token::EndOfFile, _)) => return Ok(()),
                    Some(token_s) => {
                        debug!("[COMPILER] getting `infix_rule` for: {:?}", prev);
                        Compiler::get_rule(token_s)?
                            .infix
                            .ok_or_else(|| (InternalError::UnexpectedCodePath.into(), NO_SPAN))?
                    }
                }
            };

            infix_rule(self)?;
        }
        Ok(())
    }

    fn unary(&mut self) -> Result<(), LoxErrorS> {
        let unary_kind = self.parser.prev.clone();

        self.parse_precedence(precedence::PREC_UNARY)?;

        match unary_kind {
            Some((token, span)) => match token {
                Token::Minus => {
                    self.emit_byte((opcode::NEGATE, span.clone()))?;
                    Ok(())
                }
                _ => Err((InternalError::UnexpectedCodePath.into(), span.clone())),
            },
            _ => Err((InternalError::UnexpectedCodePath.into(), NO_SPAN)),
        }
    }

    fn binary(&mut self) -> Result<(), LoxErrorS> {
        let binary_kind = self.parser.prev.clone();
        let precedence = {
            let prev = self
                .parser
                .prev
                .as_ref()
                .ok_or_else(|| (InternalError::UnexpectedCodePath.into(), NO_SPAN))?;
            Compiler::get_rule(prev)?.precedence
        };

        self.parse_precedence(precedence + 1)?;

        match binary_kind {
            Some((token, span)) => match token {
                Token::Plus => self.emit_byte((opcode::ADD, span.clone())),
                Token::Minus => self.emit_byte((opcode::SUBTRACT, span.clone())),
                Token::Star => self.emit_byte((opcode::MULTIPLY, span.clone())),
                Token::Slash => self.emit_byte((opcode::DIVIDE, span.clone())),
                _ => Err((InternalError::UnexpectedCodePath.into(), span.clone())),
            },
            _ => Err((InternalError::UnexpectedCodePath.into(), NO_SPAN)),
        }
    }

    fn emit_byte(&mut self, byte: Span<u8>) -> Result<(), LoxErrorS> {
        self.chunk.write_chunk(byte.0, byte.1);
        Ok(())
    }

    fn emit_return(&mut self) -> Result<(), LoxErrorS> {
        self.emit_byte((opcode::RETURN, NO_SPAN))
    }

    fn emit_constant(&mut self, value: Value, span: &Range<usize>) -> Result<(), LoxErrorS> {
        debug!(
            "COMPILER] adding constant to chunk: {} at {:?}",
            value, span
        );
        self.chunk.add_constant(opcode::CONSTANT, value, span)
    }

    fn number(&mut self) -> Result<(), LoxErrorS> {
        if let Some((Token::Number(number), span)) = &self.parser.prev {
            return self.emit_constant(Value::from(number.clone()), &span.clone());
        }
        Err((InternalError::UnexpectedCodePath.into(), NO_SPAN))
    }

    fn get_rule(token: &TokenS) -> Result<&'static ParseLogic, LoxErrorS> {
        match &token.0 {
            Token::LeftParen => Ok(&ParseLogic {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::RightParen => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::LeftBrace => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::RightBrace => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Semicolon => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Comma => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Dot => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Minus => Ok(&ParseLogic {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_TERM,
            }),
            Token::Plus => Ok(&ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_TERM,
            }),
            Token::Slash => Ok(&ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_FACTOR,
            }),
            Token::Star => Ok(&ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_FACTOR,
            }),
            Token::Equal => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Less => Ok(&ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_FACTOR,
            }),
            Token::More => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::BangEqual => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::EqualEqual => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::LessEqual => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::GreaterEqual => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::And => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Or => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Literal(_) => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::String(_) => Ok(&ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            Token::Number(_) => Ok(&ParseLogic {
                prefix: Some(Compiler::number),
                infix: None,
                precedence: precedence::PREC_NONE,
            }),
            _ => Err((InternalError::UnexpectedCodePath.into(), token.1.clone())),
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
struct ParseLogic {
    prefix: Option<fn(&mut Compiler) -> Result<(), LoxErrorS>>,
    infix: Option<fn(&mut Compiler) -> Result<(), LoxErrorS>>,
    precedence: u8,
}
