use std::collections::VecDeque;

use crate::{
    error::ParserError,
    parser::{AST, ASTNode},
    tokenizer::token::Token,
};

pub struct Parser {
    tokens: VecDeque<Token>,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into(),
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.front().unwrap()
    }

    fn expect(&mut self) -> Token {
        let token = self.tokens.pop_front().expect("Expected a token");
        token
    }

    fn expect_token(&mut self, t: Token) -> Result<Token, ParserError> {
        let token = self.tokens.pop_front();
        if let Some(token) = token {
            if token == t {
                Ok(token)
            } else {
                Err(ParserError::InvalidToken)
            }
        } else {
            Err(ParserError::InvalidToken)
        }
    }

    fn expect_ident(&mut self) -> Result<String, ParserError> {
        let token = self.tokens.pop_front().ok_or(ParserError::InvalidToken);
        if let Ok(token) = token {
            match token {
                Token::Ident(name) => Ok(name),
                _ => Err(ParserError::InvalidToken),
            }
        } else {
            Err(ParserError::InvalidToken)
        }
    }

    fn expect_args(&mut self) -> Vec<String> {
        let mut args = Vec::new();
        let mut exhausted = false;

        while !exhausted {
            if let Ok(ident) = self.expect_ident() {
                args.push(ident);
                exhausted = self.expect_token(Token::Comma).is_err();
            } else {
                exhausted = true;
            }
        }

        args
    }

    fn handle_func(&mut self) -> ASTNode {
        let ident = self.expect_ident().expect("Expected ident token");
        self.expect_token(Token::LeftParen)
            .expect("Expected LeftParen");
        let args = self.expect_args();
        //self.expect_token(Token::RightParen);

        let node = ASTNode::Function {
            name: ident,
            arguments: args,
        };
        node
    }

    fn handle_ident(&mut self, name: &str) -> ASTNode {
        todo!()
    }

    pub fn parse(mut self) -> AST {
        let mut ast = AST::new();
        while !self.tokens.is_empty() {
            let token = self
                .tokens
                .pop_front()
                .expect("This should not have errored");

            let node = match token {
                Token::Func => self.handle_func(),
                Token::Ident(ident) => self.handle_ident(&ident),
                Token::EoL | Token::LeftCurlyParen | Token::RightCurlyParen => continue,
                _ => panic!("Unknown Token {token:?}"),
            };
            ast.push(node);
        }

        ast
    }
}
