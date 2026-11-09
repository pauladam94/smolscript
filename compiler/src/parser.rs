use crate::{
    ast::{AST, Associativity, Binary, Constant, ExprId, ExprNode, Method, Unary},
    lexer::Token,
    types::{TypeId, TypeNode},
};
use logos::{Lexer, Span};

pub struct Parser<'a, 'src> {
    pub ast: AST<'src>,
    lexer: &'a mut Lexer<'src, Token>,
}

impl<'a, 'src> Parser<'a, 'src> {
    pub fn new(lexer: &'a mut Lexer<'src, Token>) -> Self {
        Self {
            ast: AST::default(),
            lexer,
        }
    }
    fn peek(&self) -> Option<Token> {
        match self.lexer.clone().next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) | None => None,
        }
    }
    pub fn span(&mut self) -> Span {
        self.lexer.span()
    }
    pub fn bump(&mut self) -> Option<Token> {
        match self.lexer.next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) | None => None,
        }
    }
    fn slice(&mut self) -> &'src str {
        self.lexer.slice()
    }
}

impl<'a, 'src> Parser<'a, 'src> {
    pub fn parse_str(&mut self, s: &str) -> Option<Span> {
        let _ = self.bump()?;
        let slice = self.slice();
        (slice == s).then(|| self.span())
    }
    pub fn parse_token(&mut self, token: Token) -> Option<&'src str> {
        (self.bump()? == token).then(|| self.slice())
    }
    pub fn parse_type_tuple(&mut self) -> Option<TypeId> {
        if Token::ParenClose == self.peek()? {
            let _ = self.parse_str(")").unwrap();
            return Some(self.ast.types.alloc(TypeNode::Unit));
        }

        let head = self.parse_type()?;

        if let Token::Comma = self.peek()? {
            let _ = self.parse_str(",").unwrap();
        }

        let tail = self.parse_type_tuple()?;
        Some(self.ast.types.alloc(TypeNode::Tuple { head, tail }))
    }
    pub fn parse_type(&mut self) -> Option<TypeId> {
        match self.bump()? {
            Token::U64 => Some(self.ast.types.alloc(TypeNode::U64)),
            Token::U32 => Some(self.ast.types.alloc(TypeNode::U32)),
            Token::U16 => Some(self.ast.types.alloc(TypeNode::U16)),
            Token::U8 => Some(self.ast.types.alloc(TypeNode::U8)),
            Token::Bool => Some(self.ast.types.alloc(TypeNode::Bool)),
            Token::ParenOpen => self.parse_type_tuple(),
            Token::BracketOpen => {
                let inner_type = self.parse_type()?;
                let _ = self.parse_str("]")?;
                Some(self.ast.types.alloc(TypeNode::Array(inner_type)))
            }
            Token::QuestionMark => Some(self.ast.types.alloc(TypeNode::Unknown)),
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
    pub fn parse_binary_peek(&mut self) -> Option<Binary> {
        match self.peek()? {
            Token::Add => Some(Binary::Add),
            Token::Sub => Some(Binary::Sub),
            Token::Div => Some(Binary::Div),
            Token::Mult => Some(Binary::Mult),
            Token::EqEq => Some(Binary::Equal),
            Token::Neq => Some(Binary::NotEqual),
            Token::LessThan => Some(Binary::LessThan),
            Token::LessThanEqual => Some(Binary::LessThanEqual),
            Token::MoreThan => Some(Binary::MoreThan),
            Token::MoreThanEqual => Some(Binary::MoreThanEqual),
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
    pub fn parse_fun_args(&mut self) -> Option<ExprId> {
        // If there is no args to parse.
        if Token::ParenClose == self.peek()? {
            return Some(self.ast.exprs.alloc(ExprNode::EmptyTuple));
        }

        let arg_name = self.parse_token(Token::Identifier)?;
        let _ = self.parse_str(":")?;
        let t = self.parse_type()?;
        let arg = self.ast.exprs.alloc(ExprNode::Var(arg_name));
        let head = self.ast.exprs.alloc(ExprNode::TypeInfo { expr: arg, t });

        if Token::Comma == self.peek()? {
            let _ = self.parse_str(",")?;
        }

        let tail = self.parse_fun_args()?;
        Some(self.ast.exprs.alloc(ExprNode::Tuple { head, tail }))
    }

    pub fn parse_expr_args(&mut self) -> Option<ExprId> {
        // If there is no args to parse.
        if Token::ParenClose == self.peek()? {
            return Some(self.ast.exprs.alloc(ExprNode::EmptyTuple));
        }
        let head = self.parse_expr()?;
        if Token::Comma == self.peek()? {
            let _ = self.parse_str(",")?;
        }
        let tail = self.parse_expr_args()?;
        Some(self.ast.exprs.alloc(ExprNode::Tuple { head, tail }))
    }
    fn parse_primary_expr(&mut self) -> Option<ExprId> {
        match self.bump()? {
            Token::Fn => self.parse_decl(),
            Token::True => Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Bool(true)))),
            Token::False => Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Bool(false)))),
            Token::Number => {
                let slice = self.slice();
                match slice.parse::<u64>() {
                    Ok(i) => Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Int(i)))),
                    Err(_) => Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Float(
                        slice.parse::<f32>().unwrap(),
                    )))),
                }
            }
            Token::ParenOpen => {
                if let Some(Token::ParenClose) = self.peek() {
                    self.bump();
                    Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Unit)))
                } else {
                    let expr = self.parse_expr()?;
                    // TODO parse Tuple
                    let _ = self.parse_str(")")?;
                    Some(expr)
                }
            }
            Token::BracketOpen => {
                // TODO: parse any vector
                let _ = self.parse_str("]")?;
                Some(self.ast.exprs.alloc(ExprNode::EmptyVec))
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
                    self.ast.types.alloc(TypeNode::Unknown)
                };

                let _ = self.parse_str("=")?;
                let expr = self.parse_expr()?;

                let var = self.ast.exprs.alloc(ExprNode::Var(var));
                Some(self.ast.exprs.alloc(ExprNode::Let { var, t, expr }))
            }
            Token::Identifier => {
                let identifier = self.slice();
                match self.peek() {
                    Some(Token::BracketOpen) => {
                        let _ = self.parse_str("[")?;
                        todo!()
                        // Indexing or assign
                    }
                    Some(Token::Dot) => {
                        // method call for sure
                        let _ = self.parse_str(".")?;
                        let method = self.parse_method()?;
                        let _ = self.parse_str("(")?;
                        let args = self.parse_expr_args()?;
                        let _ = self.parse_str(")")?;

                        let index = self.ast.exprs.alloc(ExprNode::Var(identifier));
                        Some(self.ast.exprs.alloc(ExprNode::MethodCall {
                            index,
                            method,
                            args,
                        }))
                    }
                    Some(Token::ParenOpen) => {
                        let _ = self.parse_str("(")?;
                        let args = self.parse_expr_args()?;
                        let _ = self.parse_str(")")?;
                        let name = self.ast.exprs.alloc(ExprNode::Var(identifier));
                        Some(self.ast.exprs.alloc(ExprNode::FunCall { name, args }))
                    }
                    _ => Some(self.ast.exprs.alloc(ExprNode::Var(identifier))),
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
                    self.ast.exprs.alloc(ExprNode::Const(Constant::Unit))
                };

                Some(self.ast.exprs.alloc(ExprNode::If { cond, yes, no }))
            }
            Token::While => {
                let _ = self.parse_str("(")?;
                let cond = self.parse_expr()?;
                let _ = self.parse_str(")")?;
                let _ = self.parse_str("{")?;
                let body = self.parse_block()?;
                let _ = self.parse_str("}")?;

                Some(self.ast.exprs.alloc(ExprNode::While { cond, body }))
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
        let mut lookahead = if let Some(lookahead) = self.parse_binary_peek() {
            lookahead
        } else {
            return Some(lhs);
        };

        while lookahead.precedence() >= min_precedence {
            let op = self.parse_binary_peek().unwrap();
            self.bump();

            let mut rhs = if let Some(rhs) = self.parse_primary_expr() {
                rhs
            } else {
                todo!()
                // -> fail gracefully
                // return Ok(lhs);
            };

            lookahead = if let Some(lookahead) = self.parse_binary_peek() {
                lookahead
            } else {
                return Some(self.ast.exprs.alloc(ExprNode::BinaryOp { lhs, op, rhs }));
            };

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

                lookahead = if let Some(lookahead) = self.parse_binary_peek() {
                    lookahead
                } else {
                    return Some(self.ast.exprs.alloc(ExprNode::BinaryOp { lhs, op, rhs }));
                };
            }
            lhs = self.ast.exprs.alloc(ExprNode::BinaryOp { lhs, op, rhs })
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
    pub fn parse_block(&mut self) -> Option<ExprId> {
        let expr = self.parse_expr()?;
        if let Some(Token::SemiColon) = self.peek() {
            let _ = self.parse_str(";");
            let block = self.parse_block()?;
            Some(self.ast.exprs.alloc(ExprNode::Seq {
                fst: expr,
                snd: block,
            }))
        } else {
            Some(expr)
        }
    }
    pub fn parse_decl(&mut self) -> Option<ExprId> {
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
            Token::BraceOpen => self.ast.types.alloc(TypeNode::Unit),
            _ => return None,
        };

        let expr = if let Token::BraceClose = self.peek()? {
            let _ = self.bump()?;
            self.ast.exprs.alloc(ExprNode::Const(Constant::Unit))
        } else {
            let expr = self.parse_block()?;
            let _ = self.parse_str("}")?;
            expr
        };

        let name = self.ast.exprs.alloc(ExprNode::Var(name));
        Some(self.ast.exprs.alloc(ExprNode::Decl {
            name,
            args,
            return_type,
            expr,
        }))
    }
}
