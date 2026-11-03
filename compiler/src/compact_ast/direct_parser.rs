use crate::{
    compact_ast::{AST, Associativity, Binary, Constant, Decl, ExprId, ExprNode, Method, Unary},
    cst::lexer::Token,
    types::{TypeId, TypeNode},
    utils::arena::Arena,
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
        let _ = self.bump()?;
        let slice = self.slice();
        (slice == s).then(|| slice)
    }
    pub fn parse_token(&mut self, token: Token) -> Option<&'src str> {
        (self.bump()? == token).then(|| self.slice())
    }
    pub fn parse_type(&mut self) -> Option<TypeId> {
        let _ = self.bump()?;
        match self.slice() {
            "u64" => Some(self.types.alloc(TypeNode::U64)),
            "u32" => Some(self.types.alloc(TypeNode::U32)),
            "u16" => Some(self.types.alloc(TypeNode::U16)),
            "u8" => Some(self.types.alloc(TypeNode::U8)),
            "bool" => Some(self.types.alloc(TypeNode::Bool)),
            "(" => {
                // TODO also parse pair types
                let _ = self.parse_str(")")?;
                Some(self.types.alloc(TypeNode::Unit))
            }
            "[" => {
                let inner_type = self.parse_type()?;
                let _ = self.parse_str("]")?;
                Some(self.types.alloc(TypeNode::Array(inner_type)))
            }
            _ => None,
        }
    }
    pub fn parse_method(&mut self) -> Option<Method> {
        let _ = self.bump()?;
        match self.slice() {
            "replace" => Some(Method::Replace),
            "push" => Some(Method::Push),
            "pop" => Some(Method::Pop),
            "len" => Some(Method::Len),
            _ => None,
        }
    }
    pub fn parse_binary(&mut self) -> Option<Binary> {
        let _ = self.bump()?;
        match self.slice() {
            "+" => Some(Binary::Add),
            "-" => Some(Binary::Sub),
            "/" => Some(Binary::Div),
            "*" => Some(Binary::Mult),
            "==" => Some(Binary::Equal),
            "!=" => Some(Binary::NotEqual),
            "<" => Some(Binary::LessThan),
            "<=" => Some(Binary::LessThanEqual),
            ">" => Some(Binary::MoreThan),
            ">=" => Some(Binary::MoreThanEqual),
            _ => None,
        }
    }
    pub fn unary_parser(&mut self) -> Option<Unary> {
        let _ = self.bump()?;
        match self.slice() {
            "not" | "!" => Some(Unary::Not),
            _ => None,
        }
    }
    pub fn parse_index(&mut self) -> Option<ExprId> {
        todo!()
    }
    pub fn parse_fun_args(&mut self) -> Option<Vec<(&'src str, TypeId)>> {
        let mut args = Vec::new();

        loop {
            let arg = if let Some(arg) = self.parse_token(Token::Identifier) {
                arg
            } else {
                return Some(args);
            };
            let _ = self.parse_str(":")?;
            let t = self.parse_type()?;
            args.push((arg, t));

            if Token::Comma == self.peek()? {
                self.bump();
            } else {
                return Some(args);
            }
        }
    }
    pub fn parse_expr_args(&mut self) -> Option<ExprId> {
        todo!()
    }
    fn parse_primary_expr(&mut self) -> Option<ExprId> {
        match self.bump()? {
            Token::True => Some(self.exprs.alloc(ExprNode::Constant(Constant::Bool(true)))),
            Token::False => Some(self.exprs.alloc(ExprNode::Constant(Constant::Bool(false)))),
            Token::Number => {
                let slice = self.slice();
                match slice.parse::<u64>() {
                    Ok(i) => Some(self.exprs.alloc(ExprNode::Constant(Constant::Int(i)))),
                    Err(_) => Some(self.exprs.alloc(ExprNode::Constant(Constant::Float(
                        slice.parse::<f32>().unwrap(),
                    )))),
                }
            }
            Token::ParenOpen => {
                if let Some(Token::ParenClose) = self.peek() {
                    Some(self.exprs.alloc(ExprNode::Constant(Constant::Unit)))
                } else {
                    let expr = self.parse_expr()?;
                    let _ = self.parse_str(")")?;
                    Some(expr)
                }
            }
            Token::BracketOpen => {
                // TODO: parse any vector
                let _ = self.parse_str("]")?;
                Some(self.exprs.alloc(ExprNode::EmptyVec))
            }
            // ex: "let x = 3 + 4"
            // ex: "let x : u64 = 3 + 4"
            Token::Let => {
                let var = self.parse_token(Token::Identifier)?;

                // Optional type annotation
                let t = if Token::Colon == self.peek()? {
                    let _ = self.parse_str(":")?;
                    self.parse_type()?
                } else {
                    self.types.alloc(TypeNode::Unknown)
                };

                let _ = self.parse_str("=")?;
                let expr = self.parse_expr()?;

                let var = self.exprs.alloc(ExprNode::Var(var));
                Some(self.exprs.alloc(ExprNode::Let { var, t, expr }))
            }
            Token::Identifier => {
                let identifier = self.slice();
                match self.peek()? {
                    Token::BracketOpen => {
                        todo!()
                        // Indexing or assign
                    }
                    Token::Dot => {
                        // method call for sure
                        let method = self.parse_method()?;
                        let _ = self.parse_str("(")?;

                        // while let Some() = self.parse_expr() {
                        // }

                        // TODO
                        let args = self.parse_expr_args()?;
                        let _ = self.parse_str(")")?;

                        let index = self.exprs.alloc(ExprNode::Var(identifier));
                        Some(self.exprs.alloc(ExprNode::MethodCall {
                            index,
                            method,
                            args,
                        }))
                    }
                    Token::ParenOpen => {
                        // function call for sure
                        todo!()
                    }
                    _ => Some(self.exprs.alloc(ExprNode::Var(identifier))),
                }
                // more complicated
                // - assign : '[' ou '='
                // - method call : '[' ou '.'
                // - index simple : '[' ou autre chose
                // - function call : '('
                // todo!()
            }
            Token::If => {
                let _ = self.parse_str("(")?;
                let cond = self.parse_expr()?;
                let _ = self.parse_str(")")?;
                let _ = self.parse_str("{")?;
                let yes = self.parse_block()?;
                let _ = self.parse_str("}")?;

                let no = if Token::Else == self.bump()? {
                    let _ = self.parse_str("{")?;
                    let no = self.parse_block()?;
                    let _ = self.parse_str("}")?;
                    no
                } else {
                    self.exprs.alloc(ExprNode::Constant(Constant::Unit))
                };

                Some(self.exprs.alloc(ExprNode::If { cond, yes, no }))
            }
            Token::While => {
                let _ = self.parse_str("(")?;
                let cond = self.parse_expr()?;
                let _ = self.parse_str(")")?;
                let _ = self.parse_str("{")?;
                let body = self.parse_block()?;
                let _ = self.parse_str("}")?;

                Some(self.exprs.alloc(ExprNode::While { cond, body }))
            }
            _ => None,
        }
    }

    /// Pratt Parsing Algorithm following the pseudo code in this
    /// [wikipedia page](https://en.wikipedia.org/wiki/Operator-precedence_parser)
    ///
    /// Here the algorithm presented in the wikipedia article :
    ///  parse_expression()
    ///      return parse_expression_1(parse_primary(), 0)
    ///
    ///  parse_expression_1(lhs, min_precedence)
    ///      lookahead := peek_next_token()
    ///      while lookahead precedence is >= min_precedence
    ///          op := lookahead
    ///          advance to next token
    ///          rhs := parse_primary ()
    ///          lookahead := peek_next_token()
    ///          while lookahead is a binary operator whose precedence is greater
    ///                   than op's, or a right-associative operator
    ///                   whose precedence is equal to op's
    ///              rhs := parse_expression_1 (rhs, precedence of op + (1 if lookahead precedence is greater, else 0))
    ///              lookahead := peek next token
    ///          lhs := (lhs op rhs)
    ///      return lhs
    fn parse_op_rec(&mut self, mut lhs: ExprId, min_precedence: usize) -> Option<ExprId> {
        let checkpoint = self.checkpoint();
        let mut lookahead = if let Some(lookahead) = self.parse_binary() {
            lookahead
        } else {
            return Some(lhs);
        };
        self.restore(checkpoint);

        while lookahead.precedence() >= min_precedence {
            let op = self.parse_binary().unwrap();
            let mut rhs = if let Some(rhs) = self.parse_primary_expr() {
                rhs
            } else {
                todo!()
                // -> fail gracefully
                // return Ok(lhs);
            };

            let checkpoint = self.checkpoint();
            lookahead = if let Some(lookahead) = self.parse_binary() {
                lookahead
            } else {
                return Some(self.exprs.alloc(ExprNode::BinaryOp { lhs, op, rhs }));
            };
            self.restore(checkpoint);

            while lookahead.precedence() > op.precedence()
                || (lookahead.precedence() == op.precedence()
                    && lookahead.associativity() == Associativity::RightToLeft)
            {
                rhs = self.parse_op_rec(
                    rhs,
                    op.precedence()
                        + (if lookahead.precedence() > op.precedence() {
                            1
                        } else {
                            0
                        }),
                )?;

                let checkpoint = self.checkpoint();
                lookahead = if let Some(lookahead) = self.parse_binary() {
                    lookahead
                } else {
                    return Some(self.exprs.alloc(ExprNode::BinaryOp { lhs, op, rhs }));
                };
                self.restore(checkpoint);
            }
            lhs = self.exprs.alloc(ExprNode::BinaryOp { lhs, op, rhs })
        }
        Some(lhs)
    }

    // ex: x + 2
    // ex: x[12] = if (x) { 42 } else { 21 }
    fn parse_expr(&mut self) -> Option<ExprId> {
        let lhs = self.parse_primary_expr()?;
        self.parse_op_rec(lhs, 0)
    }
    // ex: let a = 4; a + b
    // ex: if (true) { a + b }; x = 2;
    fn parse_block(&mut self) -> Option<ExprId> {
        if let Some(expr) = self.parse_expr() {
            if let Some(_) = self.parse_str(";") {
                let block = self.parse_block()?;
                Some(self.exprs.alloc(ExprNode::Seq {
                    fst: expr,
                    snd: block,
                }))
            } else {
                Some(expr)
            }
        } else {
            None
        }
    }
    pub fn parse_decl(&mut self) -> Option<Decl<'src>> {
        let _ = self.parse_str("fn")?;
        let name = self.parse_token(Token::Identifier)?;

        let _ = self.parse_str("(")?;
        let args = self.parse_fun_args()?;
        let _ = self.parse_str(")")?;

        let return_type = match self.bump()? {
            Token::Arrow => {
                let return_type = self.parse_type()?;
                let _ = self.parse_str("{")?;
                return_type
            }
            Token::BracketOpen => self.types.alloc(TypeNode::Unit),
            _ => return None,
        };

        let expr = self.parse_block()?;
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
