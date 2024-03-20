use std::fmt::Display;

use crate::token::Token;


#[derive(Debug, Clone)]
pub enum Ast {
    Expression(Expression),
    Statement(Statement),
}
#[derive(Debug, Clone)]
pub enum Expression {
    Value(Value),
    BinaryOperation(Box<Expression>, Bop, Box<Expression>),
    UnaryOperation(Uop, Box<Expression>),
    If(If)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Let(Let),
}
#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub tp: Type,
    /// possibly not nessesary
    pub type_infered: bool,
    pub value: Expression,
}
// todo
#[derive(Debug, Clone)]
pub struct Enum {
}
#[derive(Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Arg>
}
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
    pub ret_tp: Type,
    pub body: Vec<Ast>
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub tp: Type,
}

// todo
#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum Value {
    F32(f32),
    I32(i32),
    Type(Type),
    Var(String),
    FnCall(FnCall),
}

#[derive(Debug, Clone)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub enum Bop {
    Plus,
    Minus,
    Slash,
    Star,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Carret,
}

#[derive(Debug, Clone)]
pub enum Uop {
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub block: Vec<Ast>,
    pub elsifs: Option<Vec<ElseIf>>,
    pub els: Option<Vec<Ast>>,
}

#[derive(Debug, Clone)]
pub struct ElseIf {
    pub condition: Box<Expression>,
    pub block: Vec<Ast>,
}

impl Bop {
    pub fn from_token(token: Token) -> Option<Bop> {
        match token {
            Token::Plus => Some(Bop::Plus),
            Token::Minus => Some(Bop::Minus),
            Token::Slash => Some(Bop::Slash),
            Token::Star => Some(Bop::Star),
            Token::Gt => Some(Bop::Gt),
            Token::Lt => Some(Bop::Lt),
            Token::GtEq => Some(Bop::GtEq),
            Token::LtEq => Some(Bop::LtEq),
            Token::Carret => Some(Bop::Carret),
            _ => None,
        }
    }
}

impl Display for ElseIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "else if {} then\n", self.condition)?;
        for ast in &self.block {
            write!(f, "{}\n", ast)?;
        }
        Ok(())
    }
}

impl Display for If {
    // pub condition: Box<Expression>,
    // pub block: Vec<Ast>,
    // pub elsifs: Option<Vec<ElseIf>>,
    // pub els: Option<Vec<Ast>>,
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {} then\n", self.condition)?;
        for ast in &self.block {
            write!(f, "{}\n", ast)?;
        }
        for elsif in self.elsifs.clone().unwrap_or(Vec::new()) {
            write!(f, "{}", elsif)?;
        }
        write!(f, "else\n")?;
        match &self.els {
            Some(b) => {
                for ast in b {
                    write!(f, "{}\n", ast)?;
                };
            },
            None => {}
        }
        Ok(())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Value(v) => write!(f, "{:?}", v),
            Expression::BinaryOperation(lhs, op, rhs) => write!(f, "{} {:?} {}", lhs, op, rhs),
            Expression::UnaryOperation(_, _) => todo!(),
            Expression::If(i) => write!(f, "{}", i),
        }
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}(", self.name)?;
        for arg in &self.args {
            write!(f, "{:?}", arg)?;
        }
        write!(f, "): {:?}\n", self.ret_tp)?;
        for ast in &self.body {
            write!(f, "{}\n", ast)?;
        }
        Ok(())
    }
}
impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Function(func) => write!(f, "{}", func),
            Statement::Struct(_) => todo!(),
            Statement::Enum(_) => todo!(),
            Statement::Let(_) => todo!(),
        }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ast::Expression(e) => write!(f, "{}", e),
            Ast::Statement(s) => write!(f, "{}", s),
        }
        
    }
}
