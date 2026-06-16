use ast::{AST, Constant, Decl, Expr, Method};
use colored::Colorize;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use crate::{
    continuation::Continuation, continuation_stack::ContinuationStack, heap::Heap, memory::Memory,
    state::State, value::Value,
};

pub struct Interpreter {
    step: u64,
    top_level: VecDeque<Decl>,
    pub current: State,
    pub continuation: ContinuationStack,
    pub memory: Memory,
}

impl Interpreter {
    pub fn new(ast: AST) -> Self {
        let AST { top_level, block } = ast;
        Self {
            step: 0,
            top_level,
            current: State::Block(block),
            continuation: ContinuationStack::EMPTY,
            memory: Memory::EMPTY,
        }
    }
    pub fn heap(&self) -> &Heap {
        &self.memory.heap
    }
    pub fn get_value(&self) -> Option<Value> {
        if self.continuation.len() == 0
            && let State::Value(value) = &self.current
        {
            Some(value.clone())
        } else {
            None
        }
    }
}

impl Interpreter {
    pub fn step_expr(expr: &mut Expr, memory: &mut Memory) -> Option<State> {
        match expr {
            Expr::Index(index) => Some(State::Value(memory.index_value(std::mem::take(index)))),
            Expr::MethodCall {
                index,
                method,
                args,
            } => match method {
                Method::Replace => {
                    let arg = args.pop_front().unwrap_or_else(|| panic!("There should be one argument to a replace method. This should be checked by type checking."));
                    memory.replace_method(std::mem::take(index), arg);
                    None
                }
                Method::Push => {
                    let arg = args.pop_front().unwrap_or_else(|| panic!("There should be one argument to a push method. This should be checked by type checking."));
                    memory.push_method(std::mem::take(index), arg);
                    Some(State::Value(Value::Constant(Constant::Unit)))
                }
                Method::Pop => {
                    assert!(
                        args.is_empty(),
                        "There should be no argument to the 'pop' method. This should be checked by type checking."
                    );
                    memory.pop_method(std::mem::take(index));
                    None
                }
            },
            Expr::FunCall { name, args } => None,
            Expr::LetBindings { var, block, t } => {
                Some(State::Value(Value::Constant(Constant::Unit)))
            }
            Expr::While { cond, block } => None,
            Expr::If { cond, yes, no } => None,
            Expr::Constant(constant) => {
                Some(State::Value(Value::Constant(std::mem::take(constant))))
            }
            Expr::BinaryOp { op, lhs, rhs } => None,
            Expr::UnaryOp { lhs: var, op } => todo!(),
            Expr::EmptyVec => {
                let addr = memory.alloc();
                Some(State::Value(Value::Address(addr)))
            }
            Expr::Block(block) => todo!(),
        }
    }
}

impl Iterator for Interpreter {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        let Self {
            step,
            top_level,
            current,
            continuation,
            memory,
        } = self;
        *step += 1;
        if let State::Value(_) = current
            && continuation.len() == 0
        {
            // We finished execution
            return None;
        }
        match current {
            State::Block(block) => match block.statements.len() {
                0 => {
                    *current = State::Value(Value::Constant(Constant::Unit));
                    Some(())
                }
                1 => {
                    let expr = block.statements.pop_front().unwrap();
                    *current = State::Expr(expr);
                    Some(())
                }
                _ => {
                    let expr = block.statements.pop_front().unwrap();
                    if let Expr::LetBindings {
                        var,
                        block: block_let,
                        t,
                    } = expr
                    {
                        continuation.stack.push(Continuation::LetCont {
                            var,
                            block: std::mem::take(block),
                        });
                        *current = State::Block(block_let);
                    } else {
                        continuation
                            .stack
                            .push(Continuation::BlockCont(std::mem::take(block)));
                        *current = State::Expr(expr);
                    }
                    Some(())
                }
            },
            State::Expr(expr) => {
                if let Some(new_state) = Self::step_expr(expr, memory) {
                    *current = new_state
                }
                Some(())
            }
            State::Value(value) => {
                if continuation.stack.len() == 0 {
                    None
                } else {
                    let cont = continuation.pop().unwrap();
                    match cont {
                        Continuation::LetCont { var, block } => {
                            memory.bind_var(var.clone(), std::mem::take(value));
                            continuation.stack.push(Continuation::DropCont(var));
                            *current = State::Block(block);
                        }
                        Continuation::BlockCont(block) => {
                            assert_eq!(
                                value,
                                &Value::Constant(Constant::Unit),
                                "Waiting for a unit value '()' before a block."
                            );
                            *current = State::Block(block);
                        }
                        Continuation::DropCont(var) => {
                            memory.drop(var);
                        }
                    }
                    Some(())
                }
            }
        }
    }
}

impl Display for Interpreter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}{}{}",
            "---\n".purple(),
            ">> At Step ".blue(),
            self.step
        )?;
        writeln!(f, "{}{}", ">> Current State :".blue(), self.current)?;
        writeln!(f, "{}{}", ">> Continuations :\n".blue(), self.continuation)?;
        writeln!(f, "{}\n{}", ">> Memory :".blue(), self.memory)?;

        Ok(())
    }
}
