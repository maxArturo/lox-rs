use std::{collections::HashMap, ops::Range};

use crate::{
    constants::NO_SPAN,
    entities::{chunk::Chunk, opcode, precedence, value::Value},
    error::{InternalError, LoxErrorS, Result},
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
    parse_rules: HashMap<Token, ParseLogic>,
}

impl Compiler {
    fn new(source: &str) -> Result<Self, Vec<LoxErrorS>> {
        let parser = Parser::new(scan(source)?);
        Ok(Self {
            chunk: Chunk::new(),
            parser,
            parse_rules: parse_rules(),
        })
    }

    fn compile(&mut self) -> Result<&Chunk, Vec<LoxErrorS>> {
        // temporarily add a return opcode
        match self.emit_return() {
            Ok(_) => Ok(&self.chunk),
            Err(e) => return Err(vec![e]),
        }
    }

    fn expression(&mut self) -> Result<(), LoxErrorS> {
        self.parse_precedence(precedence::PREC_ASSIGNMENT)?;
        todo!()
    }

    fn grouping(&mut self) -> Result<(), LoxErrorS> {
        self.expression()?;
        self.parser
            .consume(Token::RightParen, "Expected `)` after expression")
    }

    fn parse_precedence(&mut self, _prec: u8) -> Result<(), LoxErrorS> {
        todo!()
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

        // you get rule somehow, say it's 0?
        self.parse_precedence(0 + 1)?;
        match binary_kind {
            Some((token, span)) => match token {
                Token::Plus => self.emit_byte((opcode::ADD, span)),
                Token::Minus => self.emit_byte((opcode::SUBTRACT, span)),
                Token::Star => self.emit_byte((opcode::MULTIPLY, span)),
                Token::Slash => self.emit_byte((opcode::DIVIDE, span)),
                _ => Err((InternalError::UnexpectedCodePath.into(), span)),
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
        self.chunk.add_constant(opcode::CONSTANT, value, span)
    }

    fn consume_token(&mut self, (token, span): TokenS) -> Result<(), LoxErrorS> {
        match token {
            Token::Number(num) => self.emit_constant(Value::from(num), &span),
            _ => Err((InternalError::UnexpectedCodePath.into(), span)),
        }
    }

    fn number(&mut self) -> Result<(), LoxErrorS> {
        if let Some((Token::Number(number), span)) = self.parser.prev.clone() {
            return self.emit_constant(Value::from(number), &span);
        }
        Err((InternalError::UnexpectedCodePath.into(), NO_SPAN))
    }

    fn silly(&mut self) -> &'static ParseLogic {
        &ParseLogic {
            prefix: Some(|foo| {
                println!("{:?}", foo.chunk);
                // print!("{:?}", self.chunk);
                Ok(())
            }),
            infix: None,
            precedence: precedence::PREC_UNARY,
        }
    }

    fn get_rule(&mut self, token: TokenS) -> Result<&ParseLogic, LoxErrorS> {
        self.parse_rules
            .get(&token.0)
            .ok_or((InternalError::UnexpectedCodePath.into(), token.1))
    }
}

#[derive(PartialEq, PartialOrd, Clone)]
struct ParseLogic {
    prefix: Option<fn(&mut Compiler) -> Result<(), LoxErrorS>>,
    infix: Option<fn(&mut Compiler) -> Result<(), LoxErrorS>>,
    precedence: u8,
}

fn parse_rules() -> HashMap<Token, ParseLogic> {
    std::collections::HashMap::from([
        (
            Token::LeftParen,
            ParseLogic {
                prefix: Some(Compiler::grouping),
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::RightParen,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::LeftBrace,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::RightBrace,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Semicolon,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Comma,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Dot,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Minus,
            ParseLogic {
                prefix: Some(Compiler::unary),
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_TERM,
            },
        ),
        (
            Token::Plus,
            ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_TERM,
            },
        ),
        (
            Token::Slash,
            ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_FACTOR,
            },
        ),
        (
            Token::Star,
            ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_FACTOR,
            },
        ),
        (
            Token::Equal,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Less,
            ParseLogic {
                prefix: None,
                infix: Some(Compiler::binary),
                precedence: precedence::PREC_FACTOR,
            },
        ),
        (
            Token::More,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::BangEqual,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::EqualEqual,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::LessEqual,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::GreaterEqual,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::And,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Or,
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Literal("".to_owned()),
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::String("".to_owned()),
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
        (
            Token::Number(0.0),
            ParseLogic {
                prefix: None,
                infix: None,
                precedence: precedence::PREC_NONE,
            },
        ),
    ])
}
