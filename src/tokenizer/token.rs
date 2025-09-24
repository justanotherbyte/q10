use crate::parser::ASTNode;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Ident(String),
    Number(f32),
    Eq,
    Space,
    EoL,
    Add,
    Subtract,
    Multiply,
    Divide,
    Func,
    Class,
    LeftParen,
    RightParen,
    LeftCurlyParen,
    RightCurlyParen,
    Dot,
    Comma,
    Literal(String),
}

impl Into<ASTNode> for Token {
    fn into(self) -> ASTNode {
        match self {
            Self::Literal(v) => ASTNode::Literal(v),
            Self::Number(v) => ASTNode::Number(v),
            _ => unreachable!(),
        }
    }
}
