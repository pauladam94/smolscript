use crate::{
    CSTExpr,
    lexer::Token,
    parser::{
        Parser, block_parser,
        error::ParseError,
        if_else_parser, index_parser, literal_parser, method_parser,
        op::{Associativity, binary_parser},
    },
    parser_combinator::{alt::alt, cut::cut, opt::opt, repeat::repeat},
};
use alloc::{boxed::Box, string::ToString};
use colored::Colorize;
use logos::Lexer;

pub fn expr_parser<'src>(input: &mut Lexer<'src, Token>) -> Result<CSTExpr<'src>, ParseError> {
    let lhs = primary_expr_parser(input)?;
    op_parser_rec(input, lhs, 0)
}

pub fn primary_expr_parser<'src>(
    input: &mut Lexer<'src, Token>,
) -> Result<CSTExpr<'src>, ParseError> {
    alt((
        ("(", expr_parser, ")").map(|(_, expr, _)| expr),
        ("[", cut("]")).map(|_| CSTExpr::EmptyVec),
        literal_parser().map(CSTExpr::Constant),
        ("let", cut((Token::Identifier, "=", expr_parser))).map(|(_, (var, _, expr))| {
            CSTExpr::LetBindings {
                var,
                expr: Box::new(expr),
            }
        }),
        (index_parser, ":", "=", expr_parser).map(|(var, _, _, e)| CSTExpr::Assign {
            var,
            expr: Box::new(e),
        }),
        (
            "while",
            cut(("(", expr_parser, ")", "{", block_parser, "}")),
        )
            .map(|(_, (_, cond, _, _, block, _))| CSTExpr::While {
                cond: Box::new(cond),
                block,
            })
            .context("While parsing a 'while' expression".to_string()),
        if_else_parser.context("While parsing an 'if' expression".to_string()),
        (
            Token::Identifier,
            "(",
            cut((repeat(0.., (expr_parser, opt(",")).map(|(b, _)| b)), ")")),
        )
            .map(|(name, _, (args, _))| CSTExpr::FunCall { name, args })
            .context("While parsing a function call".to_string()),
        (
            index_parser,
            ".",
            cut((
                method_parser,
                "(",
                repeat(0.., (Token::Identifier, opt(",")).map(|(s, _)| s)),
                ")",
            )),
        )
            .map(|(index, _, (method, _, args, _))| CSTExpr::MethodCall {
                index,
                method,
                args,
            })
            .context("While parsing a method call".to_string()),
        index_parser
            .context("While parsing a variable or indexing".to_string())
            .map(CSTExpr::Index),
    ))
    .context("While parsing an expression".to_string())
    .parse(input)
}

/// Pratt Parsing Algorithm following the pseudo code in this
/// [wikipedia page](https://en.wikipedia.org/wiki/Operator-precedence_parser)
pub fn op_parser_rec<'src>(
    input: &mut Lexer<'src, Token>,
    mut lhs: CSTExpr<'src>,
    min_precedence: usize,
) -> Result<CSTExpr<'src>, ParseError> {
    let mut lookahead = if let Ok(lookahead) = binary_parser(&mut input.clone()) {
        lookahead
    } else {
        return Ok(lhs);
    };

    while lookahead.precedence() >= min_precedence {
        let op = binary_parser(input).unwrap();
        let mut rhs = if let Ok(rhs) = primary_expr_parser(input) {
            rhs
        } else {
            todo!()
            // -> fail gracefully
            // return Ok(lhs);
        };

        lookahead = if let Ok(lookahead) = binary_parser(&mut input.clone()) {
            lookahead
        } else {
            return Ok(CSTExpr::BinaryOp {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            });
        };

        while lookahead.precedence() > op.precedence()
            || (lookahead.precedence() == op.precedence()
                && lookahead.associativity() == Associativity::RightToLeft)
        {
            rhs = op_parser_rec(
                input,
                rhs,
                op.precedence()
                    + (if lookahead.precedence() > op.precedence() {
                        1
                    } else {
                        0
                    }),
            )?;
            lookahead = if let Ok(lookahead) = binary_parser(&mut input.clone()) {
                lookahead
            } else {
                return Ok(CSTExpr::BinaryOp {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(rhs),
                });
            };
        }
        lhs = CSTExpr::BinaryOp {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }
    Ok(lhs)
}

/*
parse_expression()
    return parse_expression_1(parse_primary(), 0)

parse_expression_1(lhs, min_precedence)
    lookahead := peek_next_token()
    while lookahead precedence is >= min_precedence
        op := lookahead
        advance to next token
        rhs := parse_primary ()
        lookahead := peek_next_token()
        while lookahead is a binary operator whose precedence is greater
                 than op's, or a right-associative operator
                 whose precedence is equal to op's
            rhs := parse_expression_1 (rhs, precedence of op + (1 if lookahead precedence is greater, else 0))
            lookahead := peek next token
        lhs := (lhs op rhs)
    return lhs
*/
