use std::error::Error;
use std::fmt::Display;

use crate::token::Token;
use crate::ast::*;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Ast>, ParserError> {
    let cur_tok = tokens.first().unwrap().clone();
    let peek_tok = tokens.get(1).unwrap().clone();
    let mut parser = Parser {
        tokens,
        cur_tok,
        peek_tok,
        idx: 1,
    };
    return parser.parse()
}

#[derive(Debug)]
pub struct ParserError {
    msg: String,
    line: i32,
}
impl ParserError {
    fn with_msg<T: ToString>(msg: T) -> ParserError {
        ParserError {
            msg: msg.to_string(),
            line: 0,
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl Error for ParserError {}

struct Parser {
    tokens: Vec<Token>,
    cur_tok: Token,
    peek_tok: Token,
    idx: usize,
}

impl Parser {
    fn parse(&mut self) -> Result<Vec<Ast>, ParserError>{
        let mut program = Vec::new();
        while ! self.cur_tok.is_eof() {
            program.push(self.parse_section()?);
        }
        Ok(program)
    }
    fn parse_block(&mut self) -> Result<Vec<Ast>, ParserError> {
        if self.cur_tok != Token::LBrace {
            return Err(ParserError::with_msg("expected a block"))
        }
        self.advance();
        let mut block = Vec::new();
        while self.cur_tok != Token::RBrace && ! self.cur_tok.is_eof() {
            block.push(self.parse_section()?);
        }
        self.advance();
        return Ok(block);
    }
    fn parse_section(&mut self) -> Result<Ast, ParserError> {
        match &self.cur_tok {
            Token::Fn => Ok(Ast::Statement(self.parse_fn_def()?)),
            t => unreachable!("{:?}", t),
        }
    }
    fn parse_fn_def(&mut self) -> Result<Statement, ParserError> {
        assert!(self.cur_tok == Token::Fn);
        let name = match self.advance() {
            Token::Ident(n) => n,
            _ => unreachable!(),
        };
        self.advance();
        let args = self.parse_args_def()?;

        let ret_tp = match self.cur_tok {
            Token::Colin => {
                self.parse_type()?
            }
            _ => Type {},
        };

        let body = self.parse_block()?;

        
        return Ok(Statement::Function(Function {
            name,
            args,
            ret_tp,
            body,
        }));
    }
    fn parse_args_def(&mut self) -> Result<Vec<Arg>, ParserError> {
        if self.cur_tok != Token::LPeren {
            return Err(ParserError::with_msg("expected opening parenthesis"))
        }
        self.advance();
        let mut args = Vec::new();
        while self.cur_tok != Token::RPeren && !self.cur_tok.is_eof() {
            args.push(self.parse_arg_def()?);
        }
        self.advance();
        return Ok(args);
    }
    fn parse_arg_def(&mut self) -> Result<Arg, ParserError> {
        let name = match &self.cur_tok {
            Token::Ident(name) => name.clone(),
            t => unreachable!("{:?}", t),
        };
        if self.advance() != Token::Colin {
            return Err(ParserError::with_msg("expected type"))
        }
        let tp = self.parse_type()?;

        if self.cur_tok != Token::Coma {
            if self.cur_tok != Token::RPeren {
                return Err(ParserError::with_msg("expected coma"))
            }
        } else {
            if self.peek_tok == Token::RPeren {
                return Err(ParserError::with_msg("unexpected coma"))
            }
            self.advance();
        }

        return Ok(Arg {
            name,
            tp
        })
    }
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        self.advance();
        Ok(Type {})
    }
    fn advance(&mut self) -> Token {
        if self.peek_tok.is_eof() {
            self.cur_tok = Token::Eof;
            return Token::Eof;
        }
        self.cur_tok = self.peek_tok.clone();
        self.idx += 1;
        self.peek_tok = self.tokens.get(self.idx).unwrap().clone();
        self.cur_tok.clone()
    }
}
