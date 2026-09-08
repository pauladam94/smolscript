use crate::{
    continuation::Continuation, continuation_stack::ContinuationStack, memory::Memory,
    state::State, value::Value,
};
use ast::{AST, Constant, Decl, Expr, Method, VarIndexing};
use colored::Colorize;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone)]
pub struct Interpreter {
    history: Vec<(State, ContinuationStack, Memory)>,
    pub step: usize,
    top_level: VecDeque<Decl>,
    pub state: State,
    pub continuation: ContinuationStack,
    pub memory: Memory,
}

impl Interpreter {
    pub fn new(ast: AST) -> Self {
        let AST {
            top_level,
            expr: block,
        } = ast;
        Self {
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
            Expr::Index(index) => Some(State::Value(memory.get_index_value(std::mem::take(index)))),
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
                    // todo recup value of the pop
                    memory.pop_method(std::mem::take(index));
                    None
                }
                Method::Len => {
                    assert!(
                        args.is_empty(),
                        "There should be no argument to the 'len' method. This should be checked by type checking."
                    );
                    memory.len_method(std::mem::take(index));
                    None
                }
            },
            Expr::FunCall { name, args } => {
                let fun_decl = top_level
                    .iter()
                    .find(|decl| &decl.name == name)
                    .unwrap_or_else(|| panic!("'{}' function not found in the source. This should be checked by type checking.", name));
                let mut block = VecDeque::from([]);
                block.push_front(fun_decl.expr.clone());

                for (var_call, (var_arg, t)) in args.iter().zip(fun_decl.args.iter()) {
                    block.push_front(Expr::LetBindings {
                        var: var_arg.clone(),
                        t: t.clone(),
                        expr: Box::new(Expr::Index({
                            VarIndexing {
                                var: var_call.clone(),
                                index: VecDeque::from([]),
                            }
                        })),
                    });
                }
                Some(State::Block(block))
            }
            Expr::LetBindings { .. } => {
                unreachable!("")
            }
            Expr::While { .. } => {
                unreachable!("")
            }
            Expr::If { cond, yes, no } => {
                if let Value::Constant(Constant::Bool(true)) = memory.get_value(cond) {
                    Some(State::Block(std::mem::take(yes)))
                } else if let Value::Constant(Constant::Bool(false)) = memory.get_value(cond) {
                    Some(State::Block(std::mem::take(no)))
                } else {
                    unreachable!()
                }
            }
            Expr::Constant(constant) => {
                Some(State::Value(Value::Constant(std::mem::take(constant))))
            }
            Expr::BinaryOp { op, lhs, rhs } => Some(State::Value(
                memory
                    .get_value(lhs)
                    .compute_binary(*op, memory.get_value(rhs)),
            )),
            Expr::UnaryOp { lhs, op } => {
                Some(State::Value(memory.get_value(lhs).compute_unary(*op)))
            }
            Expr::EmptyVec => Some(State::Value(Value::Array(vec![]))),
            Expr::Block(block) => Some(State::Block(std::mem::take(block))),
        }
    }
}

impl DoubleEndedIterator for Interpreter {
    fn next_back(&mut self) -> Option<Self::Item> {
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

    fn next(&mut self) -> Option<()> {
        self.history.push((
            self.state.clone(),
            self.continuation.clone(),
            self.memory.clone(),
        ));
        let Self {
            step,
            top_level,
            state,
            continuation,
            memory,
            history: _,
        } = self;
        *step += 1;
        if *step >= 10000 {
            panic!("Too much step done !!")
        }
        if let State::Value(_) = state
            && continuation.len() == 0
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
                    if let Expr::While { cond, body } = expr {
                        let new_state = cond.clone();
                        continuation
                            .stack
                            .push(Continuation::WhileCont { cond: *cond, body });
                        *state = State::Block(VecDeque::from([*new_state]));
                    } else if let Some(new_state) = Self::step_expr(&mut expr, memory, top_level) {
                        *state = new_state
                    }
                    Some(())
                }
                _ => {
                    match block.pop_front().unwrap() {
                        Expr::LetBindings {
                            var,
                            expr: block_let,
                            t,
                        } => {
                            continuation.stack.push(Continuation::LetCont {
                                var,
                                block: std::mem::take(block),
                            });
                            *state = State::Block(VecDeque::from([*block_let.clone()]));
                        }
                        Expr::While { cond, body } => {
                            let new_state = cond.clone();
                            continuation
                                .stack
                                .push(Continuation::BlockCont(std::mem::take(block)));
                            continuation
                                .stack
                                .push(Continuation::WhileCont { cond: *cond, body });
                            *state = State::Block(VecDeque::from([*new_state]));
                        }
                        expr => {
                            continuation
                                .stack
                                .push(Continuation::BlockCont(std::mem::take(block)));
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
                        Continuation::LetCont { var, block } => {
                            memory.bind_var(var.clone(), std::mem::take(value));
                            continuation.stack.push(Continuation::DropCont(var));
                            *state = State::Block(block);
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
                                    .push(Continuation::BlockCont(VecDeque::from([Expr::While {
                                        cond: Box::new(cond),
                                        body,
                                    }])));
                                *state = State::Block(new_body);
                            } else if let Value::Constant(Constant::Bool(false)) = value {
                                *state = State::Value(Value::Constant(Constant::Unit))
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
