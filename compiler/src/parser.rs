use crate::{
    ast::{AST, Associativity, Binary, Constant, ExprId, ExprNode, Method, Unary},
    lexer::Token,
    types::{TypeId, TypeNode},
    utils::error::CompilerError,
};
use alloc::{format, string::String, vec::Vec};
use logos::{Lexer, Span};

pub struct Parser<'a, 'src> {
    pub ast: AST<'src>,
    lexer: &'a mut Lexer<'src, Token>,
    pub errors: Vec<CompilerError>,
}

impl<'a, 'src> Parser<'a, 'src> {
    pub fn new(lexer: &'a mut Lexer<'src, Token>) -> Self {
        Self {
            ast: AST::default(),
            lexer,
            errors: Vec::new(),
        }
    }
    pub fn new_error(&mut self, message: String, span: Span) {
        self.errors.push(CompilerError { message, span })
    }
    pub fn eat(&mut self, token: Token) -> bool {
        if Some(token) == self.peek() {
            self.bump();
            true
        } else {
            false
        }
    }
    fn peek(&self) -> Option<Token> {
        match self.lexer.clone().next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) | None => None,
        }
    }
    pub fn span(&self) -> Span {
        self.lexer.span()
    }
    pub fn bump(&mut self) -> Option<Token> {
        match self.lexer.next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) | None => None,
        }
    }
    fn slice(&self) -> &'src str {
        self.lexer.slice()
    }
    pub fn expect(&mut self, token: Token) -> Option<Span> {
        let next_token = self.bump()?;
        let span = self.span();
        if next_token == token {
            Some(span)
        } else {
            self.new_error(
                format!("expecting token '{}' but got '{}'", token, next_token),
                span,
            );
            None
        }
    }
    // pub fn parse_str(&mut self, s: &str) -> Option<Span> {
    //     let _ = self.bump()?;
    //     let slice = self.slice();
    //     let span = self.span();
    //     if slice == s {
    //         Some(span)
    //     } else {
    //         self.new_error(format!("expecting '{}' but got '{}'", s, slice), span);
    //         None
    //     }
    // }
    pub fn parse_token(&mut self, token: Token) -> Option<&'src str> {
        let next_token = self.bump()?;
        let slice = self.slice();
        if next_token == token {
            Some(slice)
        } else {
            let span = self.span();
            self.new_error(
                format!("expecting token '{}' but got '{}'", token, next_token),
                span,
            );
            None
        }
    }
    pub fn parse_type_tuple(&mut self) -> Option<TypeId> {
        if self.eat(Token::ParenClose) {
            return Some(self.ast.types.alloc(TypeNode::Unit));
        }

        let head = self.parse_type()?;

        self.eat(Token::Comma);

        let tail = self.parse_type_tuple()?;
        Some(self.ast.types.alloc(TypeNode::Tuple { head, tail }))
    }
    pub fn parse_type(&mut self) -> Option<TypeId> {
        match self.bump() {
            Some(Token::U64) => Some(self.ast.types.alloc(TypeNode::U64)),
            Some(Token::U32) => Some(self.ast.types.alloc(TypeNode::U32)),
            Some(Token::U16) => Some(self.ast.types.alloc(TypeNode::U16)),
            Some(Token::U8) => Some(self.ast.types.alloc(TypeNode::U8)),
            Some(Token::Bool) => Some(self.ast.types.alloc(TypeNode::Bool)),
            Some(Token::ParenOpen) => self.parse_type_tuple(),
            Some(Token::BracketOpen) => {
                let inner_type = self.parse_type()?;
                self.expect(Token::BracketClose)?;
                Some(self.ast.types.alloc(TypeNode::Array(inner_type)))
            }
            Some(Token::QuestionMark) => Some(self.ast.types.alloc(TypeNode::Unknown)),
            Some(token) => {
                let span = self.span();
                self.new_error(format!("parsing a type but got '{}'", token), span);
                None
            }
            None => {
                let span = self.span();
                self.new_error(format!("parsing a type but reached end of file."), span);
                None
            }
        }
    }
    pub fn parse_method(&mut self) -> Option<Method> {
        let _ = self.bump()?;
        match self.slice() {
            "replace" => Some(Method::Replace),
            "push" => Some(Method::Push),
            "pop" => Some(Method::Pop),
            "len" => Some(Method::Len),
            s => {
                let span = self.span();
                self.new_error(format!("expecting a method name but got '{}'", s), span);
                None
            }
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
    pub fn parse_index(&mut self, lhs: ExprId) -> Option<ExprId> {
        if self.eat(Token::BracketOpen) {
            let index = self.parse_expr()?;
            self.expect(Token::BracketClose)?;
            let index = self.ast.exprs.alloc(ExprNode::Index { body: lhs, index });
            self.parse_index(index)
        } else {
            Some(lhs)
        }
    }
    pub fn parse_fun_args(&mut self, end: Token) -> Option<ExprId> {
        // If there is no args to parse.
        if end == self.peek()? {
            return Some(self.ast.exprs.alloc(ExprNode::EmptyTuple));
        }

        let arg_name = self.parse_token(Token::Identifier)?;
        self.expect(Token::Colon)?;
        let t = self.parse_type()?;
        let arg = self.ast.exprs.alloc(ExprNode::Var(arg_name));
        let head = self.ast.exprs.alloc(ExprNode::TypeInfo { expr: arg, t });

        self.eat(Token::Comma);

        let tail = self.parse_fun_args(end)?;
        Some(self.ast.exprs.alloc(ExprNode::Tuple { head, tail }))
    }

    pub fn parse_expr_args(&mut self, separator: Token, end: Token) -> Option<ExprId> {
        // If there is no args to parse.
        if end == self.peek()? {
            return Some(self.ast.exprs.alloc(ExprNode::EmptyTuple));
        }
        let head = self.parse_expr()?;
        self.eat(separator);
        let tail = self.parse_expr_args(separator, end)?;
        Some(self.ast.exprs.alloc(ExprNode::Tuple { head, tail }))
    }

    fn parse_cond_block(&mut self) -> Option<(ExprId, ExprId)> {
        self.expect(Token::ParenOpen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::ParenClose)?;
        self.expect(Token::BraceOpen)?;
        let body = self.parse_block(Token::BraceClose)?;
        self.expect(Token::BraceClose)?;
        Some((cond, body))
    }
    fn parse_primary_expr(&mut self) -> Option<ExprId> {
        match self.bump() {
            Some(Token::Fn) => self.parse_decl(),
            Some(Token::True) => Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Bool(true)))),
            Some(Token::False) => {
                Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Bool(false))))
            }
            Some(Token::Number) => {
                let slice = self.slice();
                let constant = if let Ok(i) = slice.parse::<u64>() {
                    Constant::Int(i)
                } else if let Ok(f) = slice.parse::<f32>() {
                    Constant::Float(f)
                } else {
                    let span = self.span();
                    self.new_error(format!("invalid number '{}'", slice), span);
                    return None;
                };
                Some(self.ast.exprs.alloc(ExprNode::Const(constant)))
            }
            Some(Token::ParenOpen) => {
                if let Some(Token::ParenClose) = self.peek() {
                    self.bump();
                    Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Unit)))
                } else {
                    let expr = self.parse_expr()?;
                    // TODO parse Tuple
                    self.expect(Token::ParenClose)?;
                    Some(expr)
                }
            }
            Some(Token::BracketOpen) => {
                // TODO: parse any vector
                self.expect(Token::BracketClose)?;
                Some(self.ast.exprs.alloc(ExprNode::EmptyVec))
            }
            // ex: "let x = 3 + 4"
            // ex: "let x : u64 = 3 + 4"
            Some(Token::Let) => {
                let var = self.parse_token(Token::Identifier)?;
                let var = self.ast.exprs.alloc(ExprNode::Var(var));

                // Optional type annotation

                let t = if self.eat(Token::Colon) {
                    self.parse_type()?
                } else {
                    self.ast.types.alloc(TypeNode::Unknown)
                };

                self.expect(Token::Eq)?;
                let expr = self.parse_expr()?;

                Some(self.ast.exprs.alloc(ExprNode::Let { var, t, expr }))
            }
            // ex: x[3]
            // ex: x[y + 2] = 12
            // ex: z.len()
            // ex: f(x, y, 12)
            Some(Token::Identifier) => {
                let slice = self.slice();
                let ident = self.ast.exprs.alloc(ExprNode::Var(slice));
                match self.peek() {
                    // Indexing or Assign
                    Some(Token::BracketOpen) => {
                        let index = self.parse_index(ident)?;
                        if self.eat(Token::Eq) {
                            let expr = self.parse_expr()?;
                            Some(self.ast.exprs.alloc(ExprNode::Assign { index, expr }))
                        } else {
                            Some(index)
                        }
                    }
                    // Method call or Field access
                    Some(Token::Dot) => {
                        self.expect(Token::Dot)?;
                        let method = self.parse_method()?;
                        self.expect(Token::ParenOpen)?;
                        let args = self.parse_expr_args(Token::Comma, Token::ParenClose)?;
                        self.expect(Token::ParenClose)?;

                        Some(self.ast.exprs.alloc(ExprNode::MethodCall {
                            index: ident,
                            method,
                            args,
                        }))
                    }
                    // Function Call
                    Some(Token::ParenOpen) => {
                        self.expect(Token::ParenOpen)?;
                        let args = self.parse_expr_args(Token::Comma, Token::ParenClose)?;
                        self.expect(Token::ParenClose)?;
                        Some(
                            self.ast
                                .exprs
                                .alloc(ExprNode::FunCall { name: ident, args }),
                        )
                    }
                    Some(Token::Eq) => {
                        self.expect(Token::Eq)?;
                        let expr = self.parse_expr()?;
                        Some(
                            self.ast
                                .exprs
                                .alloc(ExprNode::Assign { index: ident, expr }),
                        )
                    }
                    // Simple Identifier
                    _ => Some(ident),
                }
            }
            Some(Token::If) => {
                let (cond, yes) = self.parse_cond_block()?;
                let no = if self.eat(Token::Else) {
                    self.expect(Token::BraceOpen)?;
                    let no = self.parse_block(Token::BraceClose)?;
                    self.expect(Token::BraceClose)?;
                    no
                } else {
                    self.ast.exprs.alloc(ExprNode::Const(Constant::Unit))
                };

                Some(self.ast.exprs.alloc(ExprNode::If { cond, yes, no }))
            }
            Some(Token::While) => {
                let (cond, body) = self.parse_cond_block()?;
                Some(self.ast.exprs.alloc(ExprNode::While { cond, body }))
            }
            Some(token) => {
                let span = self.span();
                self.new_error(format!("parsing an expression but got '{}'", token), span);
                None
            }
            None => {
                let span = self.span();
                self.new_error(
                    format!("parsing an expression but reached end of file."),
                    span,
                );
                None
            }
        }
    }

    /// Other version of the pratt parsing:
    ///
    ///
    // fn parse_expr_bp(&mut self, min_prec: usize) -> Option<ExprId> {
    //     let mut lhs = self.parse_primary_expr()?;
    //     while let Some(op) = self.parse_binary_peek() {
    //         if op.precedence() < min_prec { break; }
    //         self.bump();
    //         let next_min = match op.associativity() {
    //             Associativity::LeftToRight => op.precedence() + 1,
    //             Associativity::RightToLeft => op.precedence(),
    //         };
    //         let rhs = self.parse_expr_bp(next_min)?;
    //         lhs = self.ast.exprs.alloc(ExprNode::BinaryOp { lhs, op, rhs });
    //     }
    //     Some(lhs)
    // }

    // fn parse_expr(&mut self) -> Option<ExprId> {
    //     self.parse_expr_bp(0)
    // }

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
        let Some(mut lookahead) = self.parse_binary_peek() else {
            return Some(lhs);
        };

        while lookahead.precedence() >= min_precedence {
            let op = self.parse_binary_peek().unwrap();
            self.bump();

            let Some(mut rhs) = self.parse_primary_expr() else {
                // -> fail gracefully
                // return Ok(lhs);
                todo!()
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
    pub fn parse_block(&mut self, end: Token) -> Option<ExprId> {
        if end == self.peek()? {
            return Some(self.ast.exprs.alloc(ExprNode::Const(Constant::Unit)));
        }
        let expr = self.parse_expr()?;
        if self.eat(Token::SemiColon) {
            let block = self.parse_block(end)?;
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

        self.expect(Token::ParenOpen)?;
        let args = self.parse_fun_args(Token::ParenClose)?;
        self.expect(Token::ParenClose)?;

        let return_type = match self.bump()? {
            Token::Arrow => {
                let return_type = self.parse_type()?;
                self.expect(Token::BraceOpen)?;
                return_type
            }
            Token::BraceOpen => self.ast.types.alloc(TypeNode::Unit),
            _ => return None,
        };

        let expr = if self.eat(Token::BraceClose) {
            self.ast.exprs.alloc(ExprNode::Const(Constant::Unit))
        } else {
            let expr = self.parse_block(Token::BraceClose)?;
            self.expect(Token::BraceClose)?;
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
