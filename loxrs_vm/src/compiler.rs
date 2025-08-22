use std::ops::Range;

use log::{debug, trace};

use crate::{
    constants::NO_SPAN,
    entities::{chunk::Chunk, object::ObjString, opcode, precedence, value::Value},
    error::{CompilerError, InternalError, LoxErrorS, Result},
    parser::Parser,
    scanner::{scan, Token, TokenS},
    types::Span,
    vm::intern_string,
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
        if let Err(e) = self.expression() {
            return Err(vec![e]);
        }

        match self.emit_return() {
            Ok(_) => Ok(&self.chunk),
            Err(e) => return Err(vec![e]),
        }
    }

    fn expression(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling expression()");
        self.parse_precedence(precedence::PREC_ASSIGNMENT)
    }

    fn grouping(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling grouping()");
        self.expression()?;
        self.parser
            .consume(Token::RightParen, "Expected `)` after expression")
    }

    fn ternary(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling ternary()");

        let ternary = self.parser.prev.clone();
        self.parse_precedence(precedence::PREC_TERNARY + 1)?;
        self.parser
            .consume(Token::Colon, "Expected `:` after expression")?;
        self.parse_precedence(precedence::PREC_TERNARY + 1)?;

        match ternary {
            Some((token, span)) => match token {
                Token::QuestionMark => self.emit_byte((opcode::TERNARY_LOGICAL, span.clone())),
                _ => Err((InternalError::UnexpectedCodePath.into(), span.clone())),
            },
            _ => Err((InternalError::UnexpectedCodePath.into(), NO_SPAN)),
        }
    }

    fn parse_precedence(&mut self, prec: u8) -> Result<(), LoxErrorS> {
        trace!("calling parse_precedence() with precedence of: {}", prec);
        self.parser.advance();

        let prefix_rule = {
            let prev = self.parser.prev.as_ref();
            match prev {
                None => return Ok(()),
                Some((Token::EndOfFile, _)) => return Ok(()),
                Some(token_s) => {
                    debug!("getting `prefix_rule` for: {:?}", prev);
                    get_rule(token_s)?.prefix.ok_or_else(|| {
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
                    Some(token_s) => {
                        let token_prec = get_rule(token_s)?.precedence;
                        trace!(
                            "evaluating `should_break` for {:?} with prec: {} vs top level prec of: {}",
                            curr_token,
                            token_prec,
                            prec
                        );
                        prec > token_prec
                    }
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
                        debug!("getting `infix_rule` for: {:?}", prev);
                        match get_rule(token_s)?.infix {
                            None => return Ok(()),
                            Some(rule) => rule,
                        }
                    }
                }
            };

            infix_rule(self)?;
        }
        Ok(())
    }

    fn unary(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling unary()");
        let unary_kind = self.parser.prev.clone();

        self.parse_precedence(precedence::PREC_UNARY)?;

        match unary_kind {
            Some((token, span)) => match token {
                Token::Minus => {
                    self.emit_byte((opcode::NEGATE, span.clone()))?;
                    Ok(())
                }
                Token::Bang => {
                    self.emit_byte((opcode::NOT, span.clone()))?;
                    Ok(())
                }
                _ => Err((InternalError::UnexpectedCodePath.into(), span.clone())),
            },
            _ => Err((InternalError::UnexpectedCodePath.into(), NO_SPAN)),
        }
    }

    fn binary(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling binary()");
        let binary_kind = self.parser.prev.clone();
        let precedence = {
            let prev = self
                .parser
                .prev
                .as_ref()
                .ok_or_else(|| (InternalError::UnexpectedCodePath.into(), NO_SPAN))?;
            get_rule(prev)?.precedence
        };

        self.parse_precedence(precedence + 1)?;

        match binary_kind {
            Some((token, span)) => match token {
                Token::BangEqual => {
                    self.emit_byte((opcode::EQUAL, span.clone()))?;
                    self.emit_byte((opcode::NOT, span.clone()))
                }
                Token::EqualEqual => self.emit_byte((opcode::EQUAL, span.clone())),
                Token::More => self.emit_byte((opcode::GREATER, span.clone())),
                Token::GreaterEqual => {
                    self.emit_byte((opcode::LESS, span.clone()))?;
                    self.emit_byte((opcode::NOT, span.clone()))
                }
                Token::Less => self.emit_byte((opcode::LESS, span.clone())),
                Token::LessEqual => {
                    self.emit_byte((opcode::GREATER, span.clone()))?;
                    self.emit_byte((opcode::NOT, span.clone()))
                }
                Token::Plus => self.emit_byte((opcode::ADD, span.clone())),
                Token::Minus => self.emit_byte((opcode::SUBTRACT, span.clone())),
                Token::Star => self.emit_byte((opcode::MULTIPLY, span.clone())),
                Token::Slash => self.emit_byte((opcode::DIVIDE, span.clone())),
                _ => Err((InternalError::UnexpectedCodePath.into(), span.clone())),
            },
            _ => Err((InternalError::UnexpectedCodePath.into(), NO_SPAN)),
        }
    }

    fn string(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling string()");
        match &self.parser.prev {
            Some((Token::String(str), span)) => {
                let object = Box::into_raw(Box::new(ObjString::new(intern_string(str))));
                self.emit_constant(object.into(), &span.clone())
            }
            Some((token, span)) => Err((
                CompilerError::UnimplementedType(token.to_string()).into(),
                span.clone(),
            )),
            None => Err((
                CompilerError::UnimplementedType(("NONE_FOUND").to_string()).into(),
                NO_SPAN,
            )),
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
        debug!("adding constant to chunk: {} at {:?}", value, span);
        self.chunk.add_constant(opcode::CONSTANT, value, span)
    }

    fn number(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling number()");
        if let Some((Token::Number(number), span)) = &self.parser.prev {
            return self.emit_constant(Value::from(number.clone()), &span.clone());
        }
        Err((InternalError::UnexpectedCodePath.into(), NO_SPAN))
    }
    fn literal(&mut self) -> Result<(), LoxErrorS> {
        trace!("calling literal()");
        match &self.parser.prev {
            Some((Token::True, span)) => self.emit_constant(Value::TRUE, &span.clone()),
            Some((Token::False, span)) => self.emit_constant(Value::FALSE, &span.clone()),
            Some((Token::Nil, span)) => self.emit_constant(Value::NIL, &span.clone()),
            Some((token, span)) => Err((
                CompilerError::UnimplementedType(token.to_string()).into(),
                span.clone(),
            )),
            None => Err((
                CompilerError::UnimplementedType(("NONE_FOUND").to_string()).into(),
                NO_SPAN,
            )),
        }
    }
}

fn get_rule(token: &TokenS) -> Result<&'static ParseLogic, LoxErrorS> {
    match &token.0 {
        Token::Colon => Ok(&ParseLogic {
            prefix: None,
            infix: None,
            precedence: precedence::PREC_TERNARY,
        }),
        Token::QuestionMark => Ok(&ParseLogic {
            prefix: None,
            infix: Some(Compiler::ternary),
            precedence: precedence::PREC_TERNARY,
        }),
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
        Token::Bang => Ok(&ParseLogic {
            prefix: Some(Compiler::unary),
            infix: None,
            precedence: precedence::PREC_NONE,
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
            precedence: precedence::PREC_COMPARISON,
        }),
        Token::More => Ok(&ParseLogic {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: precedence::PREC_COMPARISON,
        }),
        Token::BangEqual => Ok(&ParseLogic {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: precedence::PREC_EQUALITY,
        }),
        Token::EqualEqual => Ok(&ParseLogic {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: precedence::PREC_EQUALITY,
        }),

        Token::LessEqual => Ok(&ParseLogic {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: precedence::PREC_COMPARISON,
        }),
        Token::GreaterEqual => Ok(&ParseLogic {
            prefix: None,
            infix: Some(Compiler::binary),
            precedence: precedence::PREC_COMPARISON,
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
            prefix: Some(Compiler::string),
            infix: None,
            precedence: precedence::PREC_NONE,
        }),
        Token::Number(_) => Ok(&ParseLogic {
            prefix: Some(Compiler::number),
            infix: None,
            precedence: precedence::PREC_NONE,
        }),
        Token::True | Token::False | Token::Nil => Ok(&ParseLogic {
            prefix: Some(Compiler::literal),
            infix: None,
            precedence: precedence::PREC_NONE,
        }),

        other => Err((
            CompilerError::ParseLogicNotFound(other.to_string()).into(),
            token.1.clone(),
        )),
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
struct ParseLogic {
    prefix: Option<fn(&mut Compiler) -> Result<(), LoxErrorS>>,
    infix: Option<fn(&mut Compiler) -> Result<(), LoxErrorS>>,
    precedence: u8,
}
