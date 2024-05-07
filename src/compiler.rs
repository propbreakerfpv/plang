use std::{collections::HashMap, error::Error, fmt::Display, fs};

use crate::ast::{Ast, Constant, Expression, Function, Import, TypeConstr, Value};

pub fn compile(ast: Vec<Ast>) -> Result<String, CompilerError> {
    let mut compiler = Compiler::new(ast);
    let comped = compiler.compile()?;
    return fill_in(comped)
}

fn fill_in(input: String) -> Result<String, CompilerError> {
    let mut addr = 0;
    let mut addrs: HashMap<String, i32> = HashMap::new();
    let a = input
        .split_terminator('~')
        .map(|x| x.to_string())
        .enumerate()
        .map(|(i, x)| {
            if i % 2 == 1 {
                let (name, offset) = x.split_once("+").unwrap_or((&x, "0"));
                match addrs.get(name) {
                    Some(v) => (v + offset.parse::<i32>().unwrap()).to_string(),
                    None => {
                        addrs.insert(x, addr);
                        addr += 4;
                        (addr - 4).to_string()
                    }
                }
            } else {
                x
            }
        })
        .collect::<Vec<_>>();
    return Ok(a.join(""));
}

struct Compiler {
    ast: Vec<Ast>,
    indent: i32,
}


#[derive(Debug)]
pub struct CompilerError {
    msg: String,
    line: u32,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} on line {}", self.msg, self.line)
    }
}

impl Error for CompilerError {}

impl Compiler {
    fn new(ast: Vec<Ast>) -> Compiler {
        Compiler { 
            ast, 
            indent: 1,
        }
    }
    fn compile(&mut self) -> Result<String, CompilerError> {
        let mut out = String::from("(module\n");
        for section in self.ast.clone() {
            let a = match section {
                Ast::Expression(_) => todo!(),
                Ast::Statement(statmt) => match statmt {
                    crate::ast::Statement::Function(func) => self.compile_fn_def(func),
                    crate::ast::Statement::Struct(_) => continue,
                    crate::ast::Statement::Enum(_) => todo!(),
                    crate::ast::Statement::Let(_) => todo!(),
                    crate::ast::Statement::Import(import) => self.compile_import(import),
                },
            };
            out.push_str(&a.unwrap());
            // println!("compiled:\n{}", a.unwrap());
        }
        out.push_str(")");
        return Ok(out)
    }
    fn compile_import(&mut self, import: Import) -> Result<String, CompilerError> {
        let content = match fs::read_to_string(import.path) {
            Ok(c) => c,
            Err(e) => { 
                return Err(CompilerError {
                    msg: e.to_string(),
                    line: 0,
                });
            }
        };
        let content = content.trim();
        let content = content.trim_start_matches("(module");
        let mut content = content.trim_end_matches(')').trim().to_string();
        content.push_str("\n");

        Ok(content)
    }
    fn compile_fn_def(&mut self, func: Function) -> Result<String, CompilerError> {
        let mut out = String::from("(func $");
        out.push_str(&func.name);
        if &func.name == "main" {
            out.push_str(" (export \"_start\")");
        }
        for arg in func.args {
            out.push_str(" (param $");
            out.push_str(&arg.name);
            // for now we asume the type is stored in memeory and so we are passing a ptr
            out.push_str(" i32");
            out.push_str(")");
        }
        if &func.ret_tp.name != "()" {
            out.push_str(" (result i32)");
        }
        out.push_str("\n");

        let body = self.compile_block(func.body)?;
        out.push_str(&body);

        out.push_str(")\n");
        Ok(out)
    }
    fn compile_block(&mut self, block: Vec<Ast>) -> Result<String, CompilerError> {
        let mut out = String::new();
        for ast in block {
            let res = match ast {
                Ast::Expression(expr) => self.compile_expr(expr)?.join("\n"),
                Ast::Statement(_) => todo!(),
            };
            out.push_str(&res);
        }
        Ok(out)
    }
    fn compile_expr(&mut self, expr: Expression) -> Result<Vec<String>, CompilerError> {
        match expr {
            Expression::Value(v) => self.compile_val(v),
            Expression::BinaryOperation(_, _, _) => todo!(),
            Expression::UnaryOperation(_, _) => todo!(),
            Expression::If(_) => todo!(),
        }
    }
    fn compile_val(&mut self, value: Value) -> Result<Vec<String>, CompilerError> {
        let mut ret = Vec::new();
        match value {
            Value::F32(_) => todo!(),
            Value::I32(_) => todo!(),
            Value::TypeConstr(tp) => {
                ret.push(self.compile_type_construction(tp.clone())?);
                ret.push(format!("(i32.const ~type-{}~)", tp.name));
                Ok(ret)
            },
            Value::Var(_) => todo!(),
            Value::FnCall(fncall) => {
                let mut out = Vec::new(); //vec![format!("(call ${}", fncall.name)];

                let mut args = String::new();
                for arg in fncall.args {

                    let mut res = self.compile_expr(arg)?;
                    args.push_str(&res.pop().unwrap());
                    for res in res {
                        out.push(res);
                    }
                }

                out.push(format!("(call ${} {})", fncall.name, args));

                Ok(out)
            },
        }
    }
    fn compile_type_construction(&mut self, tp: TypeConstr) -> Result<String, CompilerError> {
        // todo: make this something that can be done to arbatrary types vea a trait
        if tp.name == "String" {
            let mut out = String::new();
            for v in &tp.values {
                match v.1 {
                    Constant::Value(_) => todo!(),
                    Constant::Arr(arr) => {
                        out.push_str(&format!("(i32.store (i32.const ~type-String~) (i32.const {}))\n", arr.len()));
                        for (idx, char) in arr.iter().enumerate() {
                            out.push_str(&format!("(i32.store (i32.const ~type-String+{}~) (i32.const {}))\n", idx + 4, char.get_number().unwrap()));
                        }
                        println!("v: {:?}", arr);
                    },
                }
            }
            return Ok(out);
        }
        todo!()
    }
}


fn indent(amount: i32) -> String {
    let mut out = String::new();
    for _ in 0..amount {
        out.push_str("    ");
    }
    out
}
