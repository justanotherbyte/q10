mod parsers;

use crate::tokenizer::token::Token;
pub use parsers::Parser;

#[derive(Debug)]
pub enum ASTNode {
    Assignment {
        variable: String,
        value: Box<ASTNode>,
    },
    Function {
        name: String,
        arguments: Vec<String>,
    },
    Operation {
        operator: Token,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Call {
        ident: Token,
        args: Vec<ASTNode>,
    },
    Number(f32),
    Literal(String),
    EntryPoint,
}

pub struct AST {
    nodes: Vec<ASTNode>,
}
impl AST {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn push(&mut self, node: ASTNode) {
        self.nodes.push(node);
    }
    pub fn execute(self) {
        println!("Nodes: {:?}", self.nodes);
    }
}
