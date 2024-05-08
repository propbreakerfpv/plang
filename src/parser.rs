use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::fs;

use crate::token::{Number, Token};
use crate::ast::*;
use crate::wat::get_exports;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Ast>, ParserError> {
    let cur_tok = tokens.first().unwrap().clone();
    let peek_tok = tokens.get(1).unwrap().clone();
    let mut parser = Parser {
        tokens,
        cur_tok,
        peek_tok,
        idx: 1,
        funcs: HashSet::new(),
        scopes: vec![HashSet::new()],
        types: HashSet::new(),
        line: 1,
    };

    parser.types.insert(String::from("i32"));

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
    funcs: HashSet<String>,
    scopes: Vec<HashSet<String>>,
    types: HashSet<String>,
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
            println!("{:?}", self.cur_tok);
            return self.make_err("expected a block")
        }
        self.advance();

        self.scopes.push(HashSet::new());
         
        let mut block = Vec::new();
        while self.cur_tok != Token::RBrace && ! self.cur_tok.is_eof() {
            block.push(self.parse_section()?);
        }
        if self.cur_tok == Token::RBrace {
            self.advance();
        } else {
            return self.make_err("expected right brace")
        }

        self.scopes.pop();

        return Ok(block);
    }
    fn parse_section(&mut self) -> Result<Ast, ParserError> {
        match &self.cur_tok {
            Token::Fn => Ok(Ast::Statement(self.parse_fn_def()?)),
            Token::Ident(_) => Ok(Ast::Expression(self.parse_expr()?)),
            Token::If => Ok(Ast::Expression(self.parse_if()?)),
            Token::Struct => Ok(Ast::Statement(self.parse_struct_def()?)),
            Token::Import => Ok(Ast::Statement(self.parse_import()?)),
            // Token::Let => Ok(Ast::Statement(self.parse_let()?)),
            t => unreachable!("{:?} line: {}", t, self.line),
        }
    }
    fn parse_let(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }
    fn parse_import(&mut self) -> Result<Statement, ParserError> {
        if self.cur_tok != Token::Import {
            return self.make_err("expected import");
        }
        self.advance();
        let path = if let Token::String(s) = &self.cur_tok {
            s.clone()
        } else {
            return self.make_err("expected path");
        };
        self.advance();

        // import all exported symboles
        let import = fs::read_to_string(&path)
            .map_err(|e| ParserError {
                msg: e.to_string(),
                line: self.line,
            });
        for (name, tp) in get_exports(import?) {
            match tp.as_str() {
                "func" => self.funcs.insert(name),
                _ => return self.make_err("unexpected type when parsing wat file")
            };
        }

        if self.cur_tok != Token::SemiColin {
            return self.make_err("missing semicolin");
        }
        self.advance();

        Ok(Statement::Import(Import {
            path,
        }))
    }
    fn parse_struct_def(&mut self) -> Result<Statement, ParserError> {
        if self.cur_tok != Token::Struct {
            return self.make_err("expected struct");
        }
        self.advance();
        let name = match &self.cur_tok {
            Token::Ident(i) => i.clone(),
            _ => return self.make_err("expected name")
        };
        self.advance();
        // todo: make this actualy do something
        while self.cur_tok != Token::RBrace {
            self.advance();
        }
        self.advance();

        self.types.insert(name.to_string());

        Ok(Statement::Struct(Struct {
            name: name.to_string(),
            fields: Vec::new(),
        }))
    }
    fn parse_if(&mut self) -> Result<Expression, ParserError> {
        if self.cur_tok != Token::If {
            return self.make_err("expected if");
        }
        self.advance();
        let condition = self.parse_expr()?;
        println!("after cond {:?}", self.cur_tok);

        let block = self.parse_block()?;

        let elsifs = self.parse_els_ifs()?;
        println!("elsifs: {:?}", elsifs);

        let els = if self.cur_tok == Token::Else {
            self.advance();
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(Expression::If(If {
            condition: Box::new(condition),
            block,
            elsifs,
            els,
        }))
    }
    fn parse_els_ifs(&mut self) -> Result<Option<Vec<ElseIf>>, ParserError> {
        println!("elsif cur_tok: {:?}, peek: {:?}", self.cur_tok, self.peek_tok);
        if self.cur_tok != Token::Else || self.peek_tok != Token::If {
            return Ok(None);
        }
        let mut elsifs = Vec::new();
        while self.cur_tok == Token::Else && self.peek_tok == Token::If && self.cur_tok.is_not_eof() {
            self.advance();
            self.advance();
            let condition = self.parse_expr()?;
            let block = self.parse_block()?;
            elsifs.push(ElseIf {
                condition: Box::new(condition),
                block,
            });
        }
        Ok(Some(elsifs))
    }
    fn parse_expr(&mut self) -> Result<Expression, ParserError> {
        let lhs = self.parse_value()?;
        self.advance();
        if ! self.cur_tok.is_operator() {
            return Ok(lhs);
        }
        let op = self.cur_tok.clone();
        self.advance();

        let rhs = self.parse_value()?;

        self.advance();

        let op = match Bop::from_token(op) {
            Some(v) => v,
            None => return self.make_err("expected binary operator"),
        };

        Ok(Expression::BinaryOperation(Box::new(lhs), op, Box::new(rhs)))
    }
    fn parse_value(&mut self) -> Result<Expression, ParserError> {
        match &self.cur_tok {
            Token::Ident(i) => {
                if self.get_var(i) {
                    Ok(Expression::Value(Value::Var(i.clone())))
                } else if self.funcs.get(i).is_some() && self.peek_tok == Token::LPeren {
                    Ok(Expression::Value(Value::FnCall(self.parse_fn_call()?)))
                } else {
                    println!("{:?}", self.funcs);
                    self.make_err(format!("unknown identifyer {:?}", i))
                }
            }
            Token::Number(n) => Ok(match n {
                Number::F32(f) => Expression::Value(Value::F32(*f)),
                Number::I32(i) => Expression::Value(Value::I32(*i)),
            }),
            Token::String(s) => {
                let mut values = HashMap::new();
                let chars = s.chars().map(|x| Constant::Value(Value::I32(x as i32))).collect::<Vec<_>>();
                values.insert(String::from("String"), Constant::Arr(chars));
                Ok(Expression::Value(Value::TypeConstr(TypeConstr {
                    name: "String".to_string(),
                    values,
                })))
            }
            t => unreachable!("{:?} on line: {}", t, self.line)
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
        self.scopes.last_mut().unwrap().insert(name.clone());

        return Ok(Arg {
            name,
            tp
        })
    }
    fn parse_type(&mut self) -> Result<Type, ParserError> {
        self.advance();
        let tp = match &self.cur_tok {
            Token::Ident(i) => {
                if self.types.get(i).is_some() {
                    Type { name: i.clone() }
                } else {
                    return  self.make_err("type not defined");
                }
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
    fn get_var(&self, name: &String) -> bool {
        self.scopes.iter().rev().find(|x| x.get(name).is_some() ).is_some()
    }
    fn make_err<T: ToString, U>(&self, msg: T) -> Result<U, ParserError> {
        Err(ParserError {
            msg: msg.to_string(),
            line: self.line,
        })
  
    }
}
