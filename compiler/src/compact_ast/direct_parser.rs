use crate::{
    compact_ast::{AST, Decl, ExprID, ExprNode}, cst::lexer::Token, types::{Type, TypeNode}, utils::arena::Arena,
};
use alloc::vec::Vec;
use logos::Lexer;

pub struct CheckPoint<'src>(Lexer<'src, Token>);

pub struct Parser<'a, 'src> {
    types: Arena<TypeNode>,
    exprs: Arena<ExprNode<'src>>,
    lexer: &'a mut Lexer<'src, Token>,
}

impl<'a, 'src> Parser<'a, 'src> {
    pub fn new(lexer: &'a mut Lexer<'src, Token>) -> Self {
        Self {
            types: Arena::new(),
            exprs: Arena::new(),
            lexer,
        }
    }
    fn peek(&self) -> Option<Token> {
        match self.lexer.clone().next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) | None => None,
        }
    }
    fn bump(&mut self) -> Option<Token> {
        match self.lexer.next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) | None => None,
        }
    }
    fn slice(&mut self) -> &'src str {
        self.lexer.slice()
    }
    fn checkpoint(&self) -> CheckPoint<'src> {
        CheckPoint(self.lexer.clone())
    }
    fn restore(&mut self, checkpoint: CheckPoint<'src>) {
        *self.lexer = checkpoint.0;
    }
}

impl<'a, 'src> Parser<'a, 'src> {
    pub fn parse_str(&mut self, s: &str) -> Option<&'src str> {
        let checkpoint = self.checkpoint();
        if self.bump().is_some() {
            let slice = self.slice();
            if slice == s {
                return Some(slice);
            }
        }
        self.restore(checkpoint);
        None
    }
    pub fn parse_token(&mut self, token: Token) -> Option<&'src str> {
        let checkpoint = self.checkpoint();
        if self.bump()? == token {
            Some(self.slice())
        } else {
            self.restore(checkpoint);
            None
        }
    }
    pub fn parse_type(&mut self) -> Option<Type> {
        todo!()
    }
    pub fn parse_args(&mut self) -> Option<Vec<(&str, Type)>> {
        let mut args = Vec::new();
        

        Some(args)
    }
    fn parse_primary_expr(&self) -> Option<ExprID>{
        todo!()
    }
    fn parse_expr(&self) -> Option<ExprID> {
        todo!()
    }
    pub fn parse_decl(&mut self) -> Option<Decl<'src>> {
        let _ = self.parse_str("fn")?;
        let name = self.parse_token(Token::Identifier)?;

        let _ = self.parse_str("(")?;
        let args = self.parse_args()?;
        let _ = self.parse_str(")")?;

        let return_type = if let Some(_) = self.parse_str("->") {
            self.parse_type()
                .unwrap_or_else(|| self.types.alloc(TypeNode::Unit))
        };

        let _ = self.parse_str("{")?;
        let expr = self.parse_expr()?;
        let _ = self.parse_str("}")?;

        Some(Decl {
            name,
            args,
            return_type,
            expr,
        })
    }

    pub fn parse_program(&mut self) -> Option<AST<'src>> {
        let mut top_level = Vec::new();
        while let Some(Token::Fn) = self.peek() {
            top_level.push(self.parse_decl()?);
        }
        let expr = self.parse_expr()?;
        Some(AST { top_level, expr })
    }
}
