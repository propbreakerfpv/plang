use std::str::Chars;

use crate::token::{Number, Token};


pub fn lex<T: ToString>(input: T) -> Vec<Token>{
    let input = input.to_string();
    let mut input = input.chars();
    let cur_char = '\0';
    let peek_char = input.next().unwrap();
    let mut lexer = Lexer {
        input,
        cur_char,
        peek_char,
    };

    lexer.lex()
}

fn get_keyword(word: String) -> Token {
    match word.as_str() {
        "let" => Token::Let,
        "fn" => Token::Fn,
        "for" => Token::For,
        "while" => Token::While,
        "if" => Token::If,
        "struct" => Token::Struct,
        "enum" => Token::Enum,
        "return" => Token::Return,
        "break" => Token::Break,
        "continue" => Token::Continue,
        "true" => Token::True,
        "false" => Token::False,
        _ => Token::Ident(word),
    }
}

struct Lexer<'a> {
    input: Chars<'a>,
    cur_char: char,
    peek_char: char,
}

impl<'a> Lexer<'a> {
    fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        match self.advance() {
            Some(_) => {
                let token = self.next_token();
                tokens.push(token);
                tokens.append(&mut self.lex());
                tokens
            }
            None => {
                tokens.push(Token::Eof);
                tokens
            }
        }
    }
    fn next_token(&mut self) -> Token {
        match self.cur_char {
            '(' => Token::LPeren,
            ')' => Token::RPeren,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '^' => Token::Carret,
            ',' => Token::Coma,
            '.' => Token::Dot,
            '=' => {
                match self.peek_char {
                    '=' => Token::EqTo,
                    _ => Token::Eq
                }
            }
            '!' => {
                match self.peek_char {
                    '=' => Token::NotEq,
                    _ => Token::Not
                }
            }
            '>' => {
                match self.peek_char {
                    '=' => Token::GtEq,
                    _ => Token::Gt
                }
            }
            '<' => {
                match self.peek_char {
                    '=' => Token::LtEq,
                    _ => Token::Lt
                }
            }
            ':' => Token::Colin,
            ';' => Token::SemiColin,
            '\'' => Token::Quote,
            '"' => {
                self.advance();
                let str = self.advance_while(|s| s != '"');
                self.advance();
                Token::String(str)
            }
            '\n' => Token::NewLine,
            char => match char {
                c if c.is_ascii_digit() => {
                    Token::Number(self.lex_number())
                }
                c if c.is_ascii_alphabetic() => {
                    let word = self.advance_while(|c| c.is_ascii_alphabetic() || c == '_');
                    get_keyword(word)
                }
                c if c.is_whitespace() => {
                    let word = self.advance_while(|c| c.is_whitespace());
                    Token::WhiteSpace(word)
                }
                _ => Token::Elegal

            }
        }
    }
    fn lex_number(&mut self) -> Number {
        let num = self.advance_while(|n| match n {
            char if char.is_ascii_digit() || char == '.' => true,
            _ => false
        });
        if num.contains('.') {
            Number::F32(num.parse().unwrap())
        } else {
            Number::I32(num.parse().unwrap())
        }
    }
    fn advance_while<F: Fn(char) -> bool>(&mut self, f: F) -> String {
        let mut res = String::new();
        res.push(self.cur_char);
        loop {
            if ! f(self.peek_char) {
                break;
            }
            if self.advance().is_none() {
                break;
            }
            res.push(self.cur_char);
        }
        return res;
    }
    fn advance(&mut self) -> Option<char> {
        if self.peek_char == '\0' {
            return None
        }
        self.cur_char = self.peek_char;
        self.peek_char = self.input.next().unwrap_or('\0');
        Some(self.cur_char)
    }
}


