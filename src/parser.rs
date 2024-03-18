use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;

use crate::token::{Number, Token};
use crate::ast::*;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Ast>, ParserError> {
    let cur_tok = tokens.first().unwrap().clone();
    let peek_tok = tokens.get(1).unwrap().clone();
    let mut parser = Parser {
        tokens,
        cur_tok,
        peek_tok,
        idx: 1,
        vars: HashSet::new(),
        funcs: HashSet::new(),
        line: 1,
    };
    return parser.parse()
}

#[derive(Debug)]
pub struct ParserError {
    msg: String,
    line: u32,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on line {}", self.msg, self.line)
    }
}
impl Error for ParserError {}

struct Parser {
    tokens: Vec<Token>,
    cur_tok: Token,
    peek_tok: Token,
    idx: usize,
    vars: HashSet<String>,
    funcs: HashSet<String>,
    line: u32,
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
            return self.make_err("expected a block")
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
            Token::Ident(_) => Ok(Ast::Expression(self.parse_expr()?)),
            t => unreachable!("{:?}", t),
        }
    }
    fn parse_expr(&mut self) -> Result<Expression, ParserError> {
        let lhs = self.parse_value()?;
        self.advance();
        return Ok(lhs);
    }
    fn parse_value(&mut self) -> Result<Expression, ParserError> {
        match &self.cur_tok {
            Token::Ident(i) => {
                if self.vars.get(i).is_some() {
                    Ok(Expression::Value(Value::Var(i.clone())))
                } else if self.funcs.get(i).is_some() && self.peek_tok == Token::LPeren {
                    Ok(Expression::Value(Value::FnCall(self.parse_fn_call()?)))
                } else {
                    self.make_err(format!("unknown identifyer {:?}", i))
                }
            }
            Token::Number(n) => Ok(match n {
                Number::F32(f) => Expression::Value(Value::F32(*f)),
                Number::I32(i) => Expression::Value(Value::I32(*i)),
            }),
            Token::String(s) => {
                Ok(Expression::Value(Value::Type(Type { name: "String".to_string() })))
            }
            t => unreachable!("{:?}", t)
        }
    }
    fn parse_fn_call(&mut self) -> Result<FnCall, ParserError> {
        let name = match &self.cur_tok {
            Token::Ident(n) => n.clone(),
            _ => return self.make_err("expected ident")
        };
        self.advance();
        let args = self.parse_args()?;
        Ok(FnCall {
            name,
            args,
        })
    }
    fn parse_args(&mut self) -> Result<Vec<Expression>, ParserError> {
        if self.cur_tok != Token::LPeren {
            return self.make_err("expected opening parenthesis")
        }
        self.advance();
        let mut args = Vec::new();
        while self.cur_tok != Token::RPeren && self.cur_tok.is_not_eof(){
            args.push(self.parse_expr()?);
        }
        if self.cur_tok != Token::RPeren {
            return self.make_err("expected clonsing parenthesis")
        }
        self.advance();
        return Ok(args);
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
            _ => Type { name: "()".to_string() },
        };

        let body = self.parse_block()?;

        self.funcs.insert(name.clone());
        
        return Ok(Statement::Function(Function {
            name,
            args,
            ret_tp,
            body,
        }));
    }
    fn parse_args_def(&mut self) -> Result<Vec<Arg>, ParserError> {
        if self.cur_tok != Token::LPeren {
            return self.make_err("expected opening parenthesis")
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
            return self.make_err("expected type")
        }
        let tp = self.parse_type()?;

        if self.cur_tok != Token::Coma {
            if self.cur_tok != Token::RPeren {
                println!("{:?}", self.cur_tok);
                return self.make_err("expected coma")
            }
        } else {
            if self.peek_tok == Token::RPeren {
                return self.make_err("unexpected coma")
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
        let tp = match &self.cur_tok {
            Token::Ident(i) => {
                Type { name: i.clone() }
            }
            _ => return self.make_err("expected type")
        };
        self.advance();
        return Ok(tp);
    }
    fn advance(&mut self) -> Token {
        if self.peek_tok.is_eof() {
            self.cur_tok = Token::Eof;
            return Token::Eof;
        }
        self.cur_tok = self.peek_tok.clone();
        self.idx += 1;
        while self.tokens.get(self.idx).unwrap() == &Token::NewLine {
            self.line += 1;
            self.idx += 1;
        }
        self.peek_tok = self.tokens.get(self.idx).unwrap().clone();
        self.cur_tok.clone()
    }
    fn make_err<T: ToString, U>(&self, msg: T) -> Result<U, ParserError> {
        Err(ParserError {
            msg: msg.to_string(),
            line: self.line,
        })
    }
}
