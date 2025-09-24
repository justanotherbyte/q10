use crate::lexer::token::Token;

pub enum ASTNode {
    Assignment {
        variable: String,
        value: Box<ASTNode>,
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
}

pub struct AST {
    root: ASTNode,
}
