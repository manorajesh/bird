use crate::lexer::{Token, TokenKind};

// Abstract Syntax Tree
#[derive(Debug)]
pub struct AST {
    tree: Vec<Node>
}

#[derive(Debug)]
pub enum Node {
    Entry,
    Return,
    Variable,
    Constant,
    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub struct Entry {
    name: String,
    args: Vec<String>,
    body: Vec<Node>,
}

#[derive(Debug)]
pub struct Return {
    value: String,
}

#[derive(Debug)]
pub struct Variable {
    name: String,
    value: String,
}

#[derive(Debug)]
pub struct Constant {
    value: String,
}

#[derive(Debug)]
pub struct Assign {
    args: Vec<Node>,
}

#[derive(Debug)]
pub struct Add {
    args: Vec<Node>,
}

#[derive(Debug)]
pub struct Subtract {
    args: Vec<Node>,
}

#[derive(Debug)]
pub struct Multiply {
    args: Vec<Node>,
}

#[derive(Debug)]
pub struct Divide {
    args: Vec<Node>,
}

#[derive(Debug)]
pub struct Modulo {
    args: Vec<Node>,
}

pub fn parse(tokens: Vec<Token>) -> AST {
    let mut ast = AST { tree: Vec::new() };
    let mut tokenptr = 0;
    let mut astptr = 0;
    
    for token in tokens {
        match token.kind {
            TokenKind::Operator => {
                match token.value {
                    "=" => {
                        let mut args: Vec<Node> = Vec::new();
                        let mut argptr = 0;
                        let mut arg = tokens[tokenptr + 1].value;
                        args.push(Node::Variable { name: arg, value: String::new() });
                    }
                }
            }
        }    
    }
    ast
}