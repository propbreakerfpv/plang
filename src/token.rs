
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Elegal,
    Eof,
    WhiteSpace(String),
    NewLine,

    LPeren,
    RPeren,
    LBrace,
    RBrace,

    Plus,
    Minus,
    Star,
    Slash,
    Carret,
    Coma,
    Dot,
    Eq,
    EqTo,
    NotEq,
    Not,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Colin,
    SemiColin,
    Quote,

    Number(Number),

    String(String),

    Ident(String),


    Let,
    Fn,
    For,
    While,
    If,
    Else,
    Struct,
    Enum,
    Return,
    Break,
    Continue,

    True,
    False
}

#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    F32(f32),
    I32(i32),
}

impl Token {
    pub fn is_eof(&self) -> bool {
        match self {
            Token::Eof => true,
            _ => false,
        }
    }
    pub fn is_not_eof(&self) -> bool {
        match self {
            Token::Eof => false,
            _ => true,
        }
    }
    pub fn is_whitespace(&self) -> bool {
        match self {
            Token::WhiteSpace(_) => true,
            _ => false,
        }
    }
    pub fn is_ident(&self) -> bool {
        match self {
            Token::Ident(_) => true,
            _ => false,
        }
    }
    pub fn is_operator(&self) -> bool {
        match self {
            Token::Plus | Token::Minus | Token::Slash | Token::Star |
            Token::Gt | Token::Lt | Token::GtEq | Token::LtEq |
            Token::Carret | Token::Not => true,
            _ => false,
        }
    }
}
