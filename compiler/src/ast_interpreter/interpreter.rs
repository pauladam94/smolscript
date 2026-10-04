use crate::ast::{AST, BinaryOp, Constant, Decl, Expr, Index, Method, MethodCall, UnaryOp};
use crate::ast_interpreter::{
    continuation::Continuation, memory::Memory, state::State, value::Value,
};
use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Interpreter {
    keep_history: bool,
    history: Vec<(State, Vec<Continuation>, Memory)>,
    pub step: usize,
    top_level: VecDeque<Decl>,
    pub state: State,
    pub continuation: Vec<Continuation>,
    pub memory: Memory,
}

impl Interpreter {
    pub fn new(ast: AST, keep_history: bool) -> Self {
        let AST { top_level, expr } = ast;
        Self {
            keep_history,
            step: 0,
            top_level,
            state: State::Expr(expr),
            continuation: Vec::new(),
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
    pub fn step_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Index(index) => self.state = State::Value(self.memory.get_index_value(index)),
            Expr::MethodCall(MethodCall {
                index,
                method,
                mut args,
            }) => match method {
                Method::Replace => {
                    let arg = args.pop_front().unwrap_or_else(|| panic!("There should be one argument to a replace method. This should be checked by type checking."));
                    self.state = State::Value(self.memory.replace_method(index, arg));
                }
                Method::Push => {
                    let arg = args.pop_front().unwrap_or_else(|| panic!("There should be one argument to a push method. This should be checked by type checking."));
                    self.memory.push_method(index, arg);
                    self.state = State::Value(Value::Constant(Constant::Unit));
                }
                Method::Pop => {
                    assert!(
                        args.is_empty(),
                        "There should be no argument to the 'pop' method. This should be checked by type checking."
                    );
                    self.state = State::Value(self.memory.pop_method(index));
                }
                Method::Len => {
                    assert!(
                        args.is_empty(),
                        "There should be no argument to the 'len' method. This should be checked by type checking."
                    );
                    self.state = State::Value(Value::Constant(Constant::Int(
                        self.memory.len_method(index) as u64,
                    )));
                }
            },
            Expr::Call { name, args } => {
                let fun_decl = self.top_level
                    .iter()
                    .find(|decl| decl.name == name)
                    .unwrap_or_else(|| panic!("'{name}' function not found in the source. This should be checked by type checking."));

                let mut expr = fun_decl.expr.clone();

                for (var_call, (var_arg, t)) in args.iter().zip(fun_decl.args.iter()) {
                    expr = Expr::Let {
                        var: *var_arg,
                        t: t.clone(),
                        expr: Box::new(Expr::Index({
                            Index {
                                var: *var_call,
                                index: VecDeque::from([]),
                            }
                        })),
                        body: Box::new(expr),
                    };
                }
                self.state = State::Expr(expr)
            }
            Expr::While { cond, body } => {
                self.state = State::Expr(Expr::If {
                    cond: cond.clone(),
                    yes: Box::new(Expr::Block(VecDeque::from([
                        body.as_ref().clone(),
                        Expr::While {
                            cond: cond.clone(),
                            body: body.clone(),
                        },
                    ]))),
                    no: Box::new(Expr::Constant(Constant::Unit)),
                })
            }
            Expr::Let { var, t, expr, body } => {
                self.state = State::Expr(expr.as_ref().clone());
                self.continuation
                    .push(Continuation::LetCont { var, expr: body })
            }
            Expr::If { cond, yes, no } => {
                self.state = State::Expr(cond.as_ref().clone());
                self.continuation
                    .push(Continuation::IfCont { yes: yes, no: no })
            }
            Expr::Constant(constant) => self.state = State::Value(Value::Constant(constant)),
            Expr::BinaryOp(BinaryOp { op, lhs, rhs }) => {
                self.state = State::Value(
                    self.memory
                        .get_value(lhs)
                        .compute_binary(op, self.memory.get_value(rhs)),
                )
            }
            Expr::UnaryOp(UnaryOp { lhs, op }) => {
                self.state = State::Value(self.memory.get_value(lhs).compute_unary(op))
            }
            Expr::EmptyVec => self.state = State::Value(Value::Array(Vec::new())),
            Expr::Block(mut block) => {
                if block.len() == 0 {
                    self.state = State::Value(Value::Constant(Constant::Unit))
                } else if block.len() == 1 {
                    let expr = block.pop_front().unwrap();
                    self.state = State::Expr(expr);
                } else {
                    let expr = block.pop_front().unwrap();
                    self.state = State::Expr(expr);
                    self.continuation.push(Continuation::SeqCont(block));
                }
            }
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
        self.step += 1;
        assert!(self.step < 10000, "Too much step done !!");
        if let State::Value(_) = self.state
            && self.continuation.is_empty()
        {
            // We finished execution
            return None;
        }
        match self.state.clone() {
            State::Expr(expr) => {
                self.step_expr(expr);
                Some(())
            }
            State::Value(value) => {
                if self.continuation.is_empty() {
                    None
                } else {
                    let cont = self.continuation.pop().unwrap();
                    match cont {
                        Continuation::LetCont { var, expr } => {
                            self.memory.bind_var(var, value);
                            self.continuation.push(Continuation::DropCont(var));
                            self.state = State::Expr(*expr);
                        }
                        Continuation::SeqCont(block) => {
                            // assert_eq!(
                            //     value,
                            //     Value::Constant(Constant::Unit),
                            //     "Waiting for a unit value '()' before a block."
                            // );
                            self.state = State::Expr(Expr::Block(block));
                        }
                        Continuation::DropCont(var) => {
                            self.memory.drop(var);
                        }
                        Continuation::IfCont { yes, no } => {
                            if let Value::Constant(Constant::Bool(true)) = value {
                                self.state = State::Expr(*yes)
                            } else if let Value::Constant(Constant::Bool(false)) = value {
                                self.state = State::Expr(*no)
                            } else {
                                unreachable!(
                                    "The inside of an if condition must be of type 'bool' but we got '{}'. Type checking should have avoided this.",
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
        writeln!(f, "{}{}{}", "---\n", ">> At Step ", self.step)?;
        writeln!(f, ">> Current State :\n{}", self.state)?;
        writeln!(
            f,
            "{}{}",
            ">> Continuations :\n",
            self.continuation.iter().rev().format("\n :: ")
        )?;
        writeln!(f, "{}\n{}", ">> Memory :", self.memory)?;

        Ok(())
    }
}
