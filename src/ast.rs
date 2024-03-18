
#[derive(Debug)]
pub enum Ast {
    Expression(Expression),
    Statement(Statement),
}
#[derive(Debug)]
pub enum Expression {
    Value(Value),
    BinaryOperation(Box<Expression>, Bop, Box<Expression>),
    UnaryOperation(Uop, Box<Expression>),
    If(If)
}

#[derive(Debug)]
pub enum Statement {
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Let(Let),
}
#[derive(Debug)]
pub struct Let {
    pub name: String,
    pub tp: Type,
    /// possibly not nessesary
    pub type_infered: bool,
    pub value: Expression,
}
// todo
#[derive(Debug)]
pub struct Enum {
}
#[derive(Debug)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Arg>
}
#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Arg>,
    pub ret_tp: Type,
    pub body: Vec<Ast>
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub tp: Type,
}

// todo
#[derive(Debug)]
pub struct Type {
    pub name: String,
}

#[derive(Debug)]
pub enum Value {
    F32(f32),
    I32(i32),
    Type(Type),
    Var(String),
    FnCall(FnCall),
}

#[derive(Debug)]
pub struct FnCall {
    pub name: String,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub enum Bop {
}

#[derive(Debug)]
pub enum Uop {
}

#[derive(Debug)]
pub struct If {
    condition: Box<Expression>,
    block: Block,
    elsifs: Vec<ElseIf>,
    els: Block,
}

#[derive(Debug)]
pub struct ElseIf {
    condition: Box<Expression>,
    block: Block,
}

#[derive(Debug)]
pub struct Block {
}


