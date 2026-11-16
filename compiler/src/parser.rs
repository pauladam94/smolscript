use crate::{
    ast::{AST, Associativity, Binary, Constant, ExprId, ExprNode, Method, Unary},
    lexer::Token,
    types::{TypeId, TypeNode},
    utils::{error::CompilerError, span::Span},
};
use ExprNode::*;
use alloc::{format, string::String, vec::Vec};
use logos::Lexer;

pub struct Parser<'a, 'src> {
    pub ast: AST<'src>,
    lexer: &'a mut Lexer<'src, Token>,
    pub errors: Vec<CompilerError>,
}

impl<'a, 'src> Parser<'a, 'src> {
    pub fn new(lexer: &'a mut Lexer<'src, Token>) -> Self {
        Self {
            ast: AST::new(lexer.source()),
            lexer,
            errors: Vec::new(),
        }
    }
    pub fn alloc_type(&mut self, t: TypeNode) -> TypeId {
        self.ast.types.alloc(t)
    }
    pub fn alloc_expr(&mut self, expr: ExprNode, span: Span) -> ExprId {
        self.ast.positions.alloc(span);
        self.ast.exprs.alloc(expr)
    }

    pub fn new_error(&mut self, message: String, span: Span) {
        self.errors.push(CompilerError {
            message: format!("[Parser Error]: {}", message),
            span,
        })
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

    pub fn bump(&mut self) -> Option<Token> {
        match self.lexer.next() {
            Some(Ok(token)) => Some(token),
            Some(Err(_)) | None => None,
        }
    }

    pub fn span(&self) -> Span {
        let lexer_span = self.lexer.span();
        core::range::Range {
            start: lexer_span.start,
            end: lexer_span.end,
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
    pub fn parse_token(&mut self, token: Token) -> Option<Span> {
        let next_token = self.bump()?;
        if next_token == token {
            Some(self.span())
        } else {
            self.new_error(
                format!("expecting token '{}' but got '{}'", token, next_token),
                self.span(),
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

    // TODO : use this function
    pub fn unary_parser(&mut self) -> Option<Unary> {
        let _ = self.bump()?;
        match self.slice() {
            "not" | "!" => Some(Unary::Not),
            _ => None,
        }
    }

    pub fn parse_index(&mut self, lhs: ExprId) -> Option<ExprId> {
        if self.eat(Token::BracketOpen) {
            let span = self.span();
            let index = self.parse_expr()?;
            self.expect(Token::BracketClose)?;
            let index = self.alloc_expr(Index { body: lhs, index }, span);
            self.parse_index(index)
        } else {
            Some(lhs)
        }
    }

    // TODO use parse_list_expr instead
    //
    pub fn parse_fun_args(&mut self, end: Token) -> Option<ExprId> {
        if self.eat(end) {
            return Some(self.alloc_expr(EmptyTuple, self.span()));
        }

        let span_name = self.parse_token(Token::Identifier)?;

        self.expect(Token::Colon)?;
        let span_colon = self.span();

        let t = self.parse_type()?;
        let arg = self.alloc_expr(Var, span_name);
        let head = self.alloc_expr(TypeInfo { expr: arg, t }, span_colon);

        self.eat(Token::Comma);
        let span_comma = self.span();

        let tail = self.parse_fun_args(end)?;
        Some(self.alloc_expr(Tuple { head, tail }, span_comma))
    }

    // Parse a list of expression given a separator and and an end token.
    // Typically used to parse function call or tuples.
    //
    // ex: x, 2, 3 + 4)
    // ex: x; if(x){true} else {false}; "hey"]
    pub fn parse_list_expr(&mut self, separator: Token, end: Token) -> Option<ExprId> {
        if self.eat(end) {
            return Some(self.alloc_expr(EmptyTuple, self.span()));
        }
        let head = self.parse_expr()?;
        self.eat(separator);
        let span_separator = self.span();
        let tail = self.parse_list_expr(separator, end)?;
        Some(self.alloc_expr(Tuple { head, tail }, span_separator))
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
            // ex: fn test() {}
            // ex:
            Some(Token::Fn) => self.parse_decl(),
            // ex: true
            Some(Token::True) => Some(self.alloc_expr(Const(Constant::Bool(true)), self.span())),
            // ex: false
            Some(Token::False) => Some(self.alloc_expr(Const(Constant::Bool(false)), self.span())),
            // ex: 2.3
            // ex: 234
            Some(Token::Number) => {
                let slice = self.slice();
                let span = self.span();
                let constant = if let Ok(i) = slice.parse::<u32>() {
                    Constant::Int(i)
                } else if let Ok(f) = slice.parse::<f32>() {
                    Constant::Float(f)
                } else {
                    self.new_error(format!("invalid number '{}'", slice), span);
                    return None;
                };
                Some(self.alloc_expr(Const(constant), span))
            }
            // ex: "hello world"
            Some(Token::Text) => Some(self.alloc_expr(Text, self.span())),
            // ex: ()
            // ex: (x + 2)
            // ex: ((!y == false))
            // TODO ex: (x, y, 2 + 3, "hey")
            Some(Token::ParenOpen) => {
                if self.eat(Token::ParenClose) {
                    let span = self.span();
                    Some(self.alloc_expr(Const(Constant::Unit), span))
                } else {
                    let expr = self.parse_expr()?;
                    // TODO parse Tuple
                    self.expect(Token::ParenClose)?;
                    Some(expr)
                }
            }
            // ex: []
            // TODO ex: [x, 2 + 3, while(x == 12) {}]
            Some(Token::BracketOpen) => {
                // TODO: parse any vector
                self.expect(Token::BracketClose)?;
                Some(self.alloc_expr(EmptyVec, self.span()))
            }
            // ex: "let x = 3 + 4"
            // ex: "let x : u64 = 3 + 4"
            Some(Token::Let) => {
                let span = self.parse_token(Token::Identifier)?;
                let var = self.alloc_expr(Var, span);

                // Optional type annotation

                let t = if self.eat(Token::Colon) {
                    self.parse_type()?
                } else {
                    self.ast.types.alloc(TypeNode::Unknown)
                };

                let span_eq = self.expect(Token::Eq)?;
                let expr = self.parse_expr()?;

                Some(self.alloc_expr(Let { var, t, expr }, span_eq))
            }
            // ex: x[3]
            // ex: x[y + 2] = 12
            // ex: z.len()
            // ex: f(x, y, 12)
            Some(Token::Identifier) => {
                let ident = self.alloc_expr(Var, self.span());
                match self.peek() {
                    // Indexing or Assign
                    Some(Token::BracketOpen) => {
                        let index = self.parse_index(ident)?;
                        if self.eat(Token::Eq) {
                            let eq_span = self.span();
                            let expr = self.parse_expr()?;
                            Some(self.alloc_expr(Assign { index, expr }, eq_span))
                        } else {
                            Some(index)
                        }
                    }
                    // Method call or Field access
                    Some(Token::Dot) => {
                        let span_dot = self.expect(Token::Dot)?;
                        let method = self.parse_method()?;
                        self.expect(Token::ParenOpen)?;
                        let args = self.parse_list_expr(Token::Comma, Token::ParenClose)?;

                        Some(self.alloc_expr(
                            MethodCall {
                                index: ident,
                                method,
                                args,
                            },
                            span_dot,
                        ))
                    }
                    // Function Call
                    Some(Token::ParenOpen) => {
                        self.expect(Token::ParenOpen)?;
                        let paren_span = self.span();
                        let args = self.parse_list_expr(Token::Comma, Token::ParenClose)?;
                        Some(self.alloc_expr(FunCall { name: ident, args }, paren_span))
                    }
                    Some(Token::Eq) => {
                        self.expect(Token::Eq)?;
                        let eq_span = self.span();
                        let expr = self.parse_expr()?;
                        Some(self.alloc_expr(Assign { index: ident, expr }, eq_span))
                    }
                    // Simple Identifier
                    _ => Some(ident),
                }
            }
            Some(Token::If) => {
                let if_span = self.span();
                let (cond, yes) = self.parse_cond_block()?;
                let no = if self.eat(Token::Else) {
                    self.expect(Token::BraceOpen)?;
                    let no = self.parse_block(Token::BraceClose)?;
                    self.expect(Token::BraceClose)?;
                    no
                } else {
                    self.alloc_expr(Const(Constant::Unit), self.span())
                };

                Some(self.alloc_expr(If { cond, yes, no }, if_span))
            }
            Some(Token::While) => {
                let while_span = self.span();
                let (cond, body) = self.parse_cond_block()?;
                Some(self.alloc_expr(While { cond, body }, while_span))
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

    /// Pratt Parsing Algorithm following the pseudo code in this
    /// [wikipedia page](https://en.wikipedia.org/wiki/Operator-precedence_parser)
    ///
    /// ex: x + 12
    /// ex: if (true) { 2 } else { 4 } == 5
    /// ex: x == 5 * 3
    fn parse_expr_bp(&mut self, min_prec: usize) -> Option<ExprId> {
        let mut lhs = self.parse_primary_expr()?;
        while let Some(op) = self.parse_binary_peek() {
            if op.precedence() < min_prec {
                break;
            }
            self.bump();
            let binary_span = self.span();
            let next_min = match op.associativity() {
                Associativity::LeftToRight => op.precedence() + 1,
                Associativity::RightToLeft => op.precedence(),
            };
            let rhs = self.parse_expr_bp(next_min)?;
            lhs = self.alloc_expr(BinaryOp { lhs, op, rhs }, binary_span);
        }
        Some(lhs)
    }

    // ex: x + 2
    // ex: x[12] = if (x) { 42 } else { 21 }
    fn parse_expr(&mut self) -> Option<ExprId> {
        self.parse_expr_bp(0)
    }

    // ex: let a = 4; a + b
    // ex: if (true) { a + b }; x = 2;
    pub fn parse_block(&mut self, end: Token) -> Option<ExprId> {
        if end == self.peek()? {
            return Some(self.alloc_expr(Const(Constant::Unit), self.span()));
        }
        let expr = self.parse_expr()?;
        if self.eat(Token::SemiColon) {
            let semicolon_span = self.span();
            let block = self.parse_block(end)?;
            Some(self.alloc_expr(
                Seq {
                    fst: expr,
                    snd: block,
                },
                semicolon_span,
            ))
        } else {
            Some(expr)
        }
    }
    pub fn parse_decl(&mut self) -> Option<ExprId> {
        let fn_span = self.span();
        let name_span = self.parse_token(Token::Identifier)?;

        self.expect(Token::ParenOpen)?;
        let args = self.parse_fun_args(Token::ParenClose)?;

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
            self.alloc_expr(Const(Constant::Unit), self.span())
        } else {
            let expr = self.parse_block(Token::BraceClose)?;
            self.expect(Token::BraceClose)?;
            expr
        };

        let name = self.alloc_expr(Var, name_span);
        Some(self.alloc_expr(
            Decl {
                name,
                args,
                return_type,
                expr,
            },
            fn_span,
        ))
    }
}
