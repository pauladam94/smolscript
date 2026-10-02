use core::fmt::{Display, Formatter};

use crate::ast::{
    AST, BinaryOp, Call, Constant, Decl, Expr, If, Index, Method, MethodCall, UnaryOp, While,
};
use crate::ast_interpreter::{
    continuation::Continuation, continuation_stack::ContinuationStack, memory::Memory,
    state::State, value::Value,
};
use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use colored::Colorize;

#[derive(Debug, Clone)]
pub struct Interpreter {
    keep_history: bool,
    history: Vec<(State, ContinuationStack, Memory)>,
    pub step: usize,
    top_level: VecDeque<Decl>,
    pub state: State,
    pub continuation: ContinuationStack,
    pub memory: Memory,
}

impl Interpreter {
    pub fn new(ast: AST, keep_history: bool) -> Self {
        let AST {
            top_level,
            expr: block,
        } = ast;
        Self {
            keep_history,
            step: 0,
            top_level,
            state: State::Block(VecDeque::from([block])),
            continuation: ContinuationStack::EMPTY,
            memory: Memory::EMPTY,
            history: Vec::new(),
        }
    }
    pub fn get_value(&self) -> Option<Value> {
        if self.continuation.is_empty()
            && let State::Value(value) = &self.state
        {
            Some(value.clone())
        } else {
            None
        }
    }
}

impl Interpreter {
    pub fn step_expr(
        expr: &mut Expr,
        memory: &mut Memory,
        top_level: &VecDeque<Decl>,
    ) -> Option<State> {
        match expr {
            Expr::Index(index) => {
                Some(State::Value(memory.get_index_value(core::mem::take(index))))
            }
            Expr::MethodCall(MethodCall {
                index,
                method,
                args,
            }) => match method {
                Method::Replace => {
                    let arg = args.pop_front().unwrap_or_else(|| panic!("There should be one argument to a replace method. This should be checked by type checking."));
                    memory.replace_method(core::mem::take(index), arg);
                    None
                }
                Method::Push => {
                    let arg = args.pop_front().unwrap_or_else(|| panic!("There should be one argument to a push method. This should be checked by type checking."));
                    memory.push_method(core::mem::take(index), arg);
                    Some(State::Value(Value::Constant(Constant::Unit)))
                }
                Method::Pop => {
                    assert!(
                        args.is_empty(),
                        "There should be no argument to the 'pop' method. This should be checked by type checking."
                    );
                    Some(State::Value(memory.pop_method(core::mem::take(index))))
                }
                Method::Len => {
                    assert!(
                        args.is_empty(),
                        "There should be no argument to the 'len' method. This should be checked by type checking."
                    );
                    memory.len_method(core::mem::take(index));
                    None
                }
            },
            Expr::Call(Call { name, args }) => {
                let fun_decl = top_level
                    .iter()
                    .find(|decl| &decl.name == name)
                    .unwrap_or_else(|| panic!("'{name}' function not found in the source. This should be checked by type checking."));
                let mut block = VecDeque::from([]);
                block.push_front(fun_decl.expr.clone());

                for (var_call, (var_arg, t)) in args.iter().zip(fun_decl.args.iter()) {
                    block.push_front(Expr::Let {
                        var: *var_arg,
                        t: t.clone(),
                        expr: Box::new(Expr::Index({
                            Index {
                                var: *var_call,
                                index: VecDeque::from([]),
                            }
                        })),
                    });
                }
                Some(State::Block(block))
            }
            Expr::Let { .. } | Expr::While(While { .. }) => {
                unreachable!("")
            }
            Expr::If(If { cond, yes, no }) => {
                if let Value::Constant(Constant::Bool(true)) = memory.get_value(*cond) {
                    Some(State::Block(core::mem::take(yes)))
                } else if let Value::Constant(Constant::Bool(false)) = memory.get_value(*cond) {
                    Some(State::Block(core::mem::take(no)))
                } else {
                    unreachable!()
                }
            }
            Expr::Constant(constant) => {
                Some(State::Value(Value::Constant(core::mem::take(constant))))
            }
            Expr::BinaryOp(BinaryOp { op, lhs, rhs }) => Some(State::Value(
                memory
                    .get_value(*lhs)
                    .compute_binary(*op, memory.get_value(*rhs)),
            )),
            Expr::UnaryOp(UnaryOp { lhs, op }) => {
                Some(State::Value(memory.get_value(*lhs).compute_unary(*op)))
            }
            Expr::EmptyVec => Some(State::Value(Value::Array(Vec::new()))),
            Expr::Block(block) => Some(State::Block(core::mem::take(block))),
        }
    }
}

impl DoubleEndedIterator for Interpreter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.keep_history {
            panic!("We choosed to not keep history here.");
        }
        let (state, continuation, memory) = self.history.pop()?;
        self.state = state;
        self.continuation = continuation;
        self.memory = memory;
        self.step -= 1;
        Some(())
    }
}

impl Iterator for Interpreter {
    type Item = ();

    // todo break this function into multiple function
    fn next(&mut self) -> Option<()> {
        if self.keep_history {
            self.history.push((
                self.state.clone(),
                self.continuation.clone(),
                self.memory.clone(),
            ));
        }
        let Self {
            keep_history: _,
            step,
            top_level,
            state,
            continuation,
            memory,
            history: _,
        } = self;
        *step += 1;
        assert!(*step < 10000, "Too much step done !!");
        if let State::Value(_) = state
            && continuation.is_empty()
        {
            // We finished execution
            return None;
        }
        match state {
            State::Block(block) => match block.len() {
                0 => {
                    *state = State::Value(Value::Constant(Constant::Unit));
                    Some(())
                }
                1 => {
                    let mut expr = block.pop_front().unwrap();
                    if let Expr::While(While { cond, body }) = expr {
                        let new_state = cond.clone();
                        continuation
                            .stack
                            .push(Continuation::WhileCont { cond: *cond, body });
                        *state = State::Block(VecDeque::from([*new_state]));
                    } else if let Some(new_state) = Self::step_expr(&mut expr, memory, top_level) {
                        *state = new_state;
                    }
                    Some(())
                }
                _ => {
                    match block.pop_front().unwrap() {
                        Expr::Let { var, expr, t } => {
                            continuation.stack.push(Continuation::LetCont {
                                var,
                                block: core::mem::take(block),
                            });
                            *state = State::Block(VecDeque::from([*expr.clone()]));
                        }
                        Expr::While(While { cond, body }) => {
                            let new_state = cond.clone();
                            continuation
                                .stack
                                .push(Continuation::BlockCont(core::mem::take(block)));
                            continuation
                                .stack
                                .push(Continuation::WhileCont { cond: *cond, body });
                            *state = State::Block(VecDeque::from([*new_state]));
                        }
                        expr => {
                            continuation
                                .stack
                                .push(Continuation::BlockCont(core::mem::take(block)));
                            *state = State::Block(VecDeque::from([expr]));
                        }
                    }
                    Some(())
                }
            },
            State::Value(value) => {
                if continuation.stack.is_empty() {
                    None
                } else {
                    let cont = continuation.pop().unwrap();
                    match cont {
                        Continuation::LetCont { var, block: expr } => {
                            memory.bind_var(var, core::mem::take(value));
                            continuation.stack.push(Continuation::DropCont(var));
                            *state = State::Block(expr);
                        }
                        Continuation::BlockCont(block) => {
                            assert_eq!(
                                value,
                                &Value::Constant(Constant::Unit),
                                "Waiting for a unit value '()' before a block."
                            );
                            *state = State::Block(block);
                        }
                        Continuation::DropCont(var) => {
                            memory.drop(var);
                        }
                        Continuation::WhileCont { cond, body } => {
                            if let Value::Constant(Constant::Bool(true)) = value {
                                let new_body = body.clone();
                                continuation
                                    .stack
                                    .push(Continuation::BlockCont(VecDeque::from([Expr::While(
                                        While {
                                            cond: Box::new(cond),
                                            body,
                                        },
                                    )])));
                                *state = State::Block(new_body);
                            } else if let Value::Constant(Constant::Bool(false)) = value {
                                *state = State::Value(Value::Constant(Constant::Unit));
                            } else {
                                unreachable!(
                                    "The inside a while condition must of type 'bool' but we got '{}'. Type checking should have avoided this.",
                                    value
                                )
                            }
                        }
                    }
                    Some(())
                }
            }
        }
    }
}

impl Display for Interpreter {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "{}{}{}",
            "---\n".purple(),
            ">> At Step ".blue(),
            self.step
        )?;
        writeln!(f, "{}{}", ">> Current State :".blue(), self.state)?;
        writeln!(f, "{}{}", ">> Continuations :\n".blue(), self.continuation)?;
        writeln!(f, "{}\n{}", ">> Memory :".blue(), self.memory)?;

        Ok(())
    }
}
