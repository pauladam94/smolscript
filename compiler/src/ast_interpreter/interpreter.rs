use crate::{
    api::SmolAPI,
    ast::{AST, Constant, ExprId, ExprNode, Method},
    ast_interpreter::{memory::Memory, value::Value},
};
use alloc::{collections::VecDeque, vec::Vec};

pub type PC = ExprId;

#[derive(Debug, Clone)]
pub struct Interpreter<'src, T: SmolAPI> {
    keep_history: bool,
    history: Vec<(PC, Memory<'src>)>,

    pub ast: AST<'src>,
    pub step: usize,

    pub previous_pc: PC,
    pub program_counter: PC,
    pub call_stack: Vec<ExprId>,
    pub values_stack: Vec<Value>,

    pub fun_decl: Vec<ExprId>,

    pub memory: Memory<'src>,

    api: T,
}

impl<'src, T: SmolAPI> Interpreter<'src, T> {
    pub fn new(ast: AST<'src>, keep_history: bool, api: T) -> Self {
        let program_counter = ast.expr;
        Self {
            keep_history,
            history: Vec::new(),

            ast,
            step: 0,

            previous_pc: program_counter,
            program_counter,
            call_stack: Vec::new(),
            values_stack: Vec::new(),

            fun_decl: Vec::new(),

            memory: Memory::EMPTY,

            api,
        }
    }
    pub fn get_value(&mut self) -> Option<Value> {
        if self.values_stack.len() == 1 {
            Some(self.values_stack.pop().unwrap())
        } else {
            None
        }
    }
    pub fn jump(&mut self, pc: PC) {
        self.previous_pc = self.program_counter;
        self.program_counter = pc;
    }
    pub fn call(&mut self, pc: PC) {
        self.call_stack.push(self.program_counter);
        self.jump(pc);
    }
    pub fn ret(&mut self) -> Option<()> {
        if self.call_stack.is_empty() {
            None
        } else {
            self.previous_pc = self.program_counter;
            self.program_counter = self.call_stack.pop().unwrap();
            Some(())
        }
    }

    fn get_fun(&self, function_name: ExprId) -> ExprId {
        let ExprNode::Var(function_name) = self.ast.exprs[function_name] else {
            panic!("Should have been prevented by type checking.")
        };
        let function_id = self
            .fun_decl
            .iter()
            .rev()
            .find(|fun_id| {
                let ExprNode::Decl { name, .. } = self.ast.exprs[**fun_id] else {
                    panic!("Should have been prevented by type checking.")
                };
                let ExprNode::Var(name) = self.ast.exprs[name] else {
                    panic!("Should have been prevented by type checking.")
                };
                name == function_name
            })
            .unwrap();
        *function_id
    }

    fn get_var_from_index(&self, mut index: ExprId) -> &'src str {
        while let ExprNode::Index { body, .. } = self.ast.exprs[index] {
            index = body;
        }
        let ExprNode::Var(v) = self.ast.exprs[index] else {
            panic!("Should have been prevented by type checking.")
        };
        v
    }
}

impl<T: SmolAPI> DoubleEndedIterator for Interpreter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.keep_history {
            panic!("We choosed to not keep history here.");
        }
        let (pc, memory) = self.history.pop()?;
        self.program_counter = pc;
        self.memory = memory;
        self.step -= 1;
        Some(())
    }
}

impl<T: SmolAPI> Iterator for Interpreter<'_, T> {
    type Item = ();

    // todo break this function into multiple function
    fn next(&mut self) -> Option<()> {
        self.step += 1;
        use crate::ast::ExprNode::*;
        match self.ast.exprs[self.program_counter] {
            ExprNode::Decl { .. } => {
                self.fun_decl.push(self.program_counter);
                self.values_stack.push(Value::Constant(Constant::Unit));
                self.ret()
            }
            ExprNode::Var(v) => {
                self.values_stack.push(self.memory.get_value(v));
                self.ret()
            }
            ExprNode::Seq { fst, snd } => {
                if self.previous_pc == fst {
                    self.call(snd);
                    Some(())
                } else if self.previous_pc == snd {
                    let snd_value = self.values_stack.pop().unwrap();
                    let _fst_value = self.values_stack.pop().unwrap();

                    self.values_stack.push(snd_value);
                    if let ExprNode::Let { .. } = self.ast.exprs[fst] {
                        self.memory.pop_var();
                    } else if let ExprNode::Decl { .. } = self.ast.exprs[fst] {
                        self.fun_decl.pop();
                    }
                    self.ret()
                } else {
                    self.call(fst);
                    Some(())
                }
            }
            ExprNode::Array { head, tail } => {
                if self.previous_pc == head {
                    self.call(tail);
                    Some(())
                } else if self.previous_pc == tail {
                    let tail_value = self.values_stack.pop().unwrap();
                    let head_value = self.values_stack.pop().unwrap();

                    if let Value::Array(mut arr) = tail_value {
                        arr.push_front(head_value);
                        self.values_stack.push(Value::Array(arr));
                    } else {
                        self.values_stack
                            .push(Value::Array(VecDeque::from([head_value, tail_value])));
                    }
                    self.ret()
                } else {
                    self.call(head);
                    Some(())
                }
            }
            ExprNode::Tuple { head, tail } => {
                if self.previous_pc == head {
                    self.call(tail);
                    Some(())
                } else if self.previous_pc == tail {
                    let tail_value = self.values_stack.pop().unwrap();
                    let head_value = self.values_stack.pop().unwrap();

                    if let Value::Array(mut arr) = tail_value {
                        arr.push_front(head_value);
                        self.values_stack.push(Value::Array(arr));
                    } else {
                        self.values_stack
                            .push(Value::Array(VecDeque::from([head_value, tail_value])));
                    }
                    self.ret()
                } else {
                    self.call(head);
                    Some(())
                }
            }
            ExprNode::Index { body, index } => {
                if let ExprNode::Var(_) = self.ast.exprs[index] {
                    if self.previous_pc == index {
                        let index_value = self.values_stack.pop().unwrap();
                        self.values_stack
                            .push(Value::Array(VecDeque::from([index_value])));
                        self.ret()
                    } else {
                        self.call(index);
                        Some(())
                    }
                } else {
                    if self.previous_pc == body {
                        self.call(index);
                        Some(())
                    } else if self.previous_pc == index {
                        let index_value = self.values_stack.pop().unwrap();
                        let body_values = self.values_stack.pop().unwrap();
                        let Value::Array(mut body_values) = body_values else {
                            panic!("This should be prevented by type checking.")
                        };
                        body_values.push_back(index_value);
                        self.values_stack.push(Value::Array(body_values));
                        self.ret()
                    } else {
                        self.call(body);
                        Some(())
                    }
                }
            }
            ExprNode::MethodCall {
                index,
                method,
                args,
            } => {
                if self.previous_pc == index {
                    self.call(index);
                    Some(())
                } else if self.previous_pc == args {
                    // TODO fix this
                    let var = ExprId(0);
                    match method {
                        Method::Push => {
                            self.memory.push_method(index, var);
                        }
                        Method::Pop => {
                            self.memory.pop_method(index);
                        }
                        Method::Replace => {
                            todo!()
                            // self.memory.replace_method(index, var, todo!());
                        }
                        Method::Len => {
                            self.memory.len_method(index);
                        }
                    };
                    self.ret()
                } else {
                    self.call(args);
                    Some(())
                }
            }
            ExprNode::FunCall {
                name,
                args: args_ptr,
            } => {
                let fun_id = self.get_fun(name);
                let ExprNode::Decl {
                    name: _,
                    mut args,
                    return_type: _,
                    expr,
                } = self.ast.exprs[fun_id]
                else {
                    panic!("This should be prevented by type checking.")
                };
                if self.previous_pc == expr {
                    while let ExprNode::Tuple { head: _, tail } = self.ast.exprs[args] {
                        self.memory.pop_var();
                        args = tail;
                    }
                    self.ret()
                } else if self.previous_pc == args_ptr {
                    let Value::Array(args_value) = self.values_stack.pop().unwrap() else {
                        panic!("This should be prevented by type checking.")
                    };
                    for value in args_value.into_iter() {
                        if let ExprNode::Tuple { head, tail } = self.ast.exprs[args]
                            && let ExprNode::TypeInfo { expr, t: _ } = self.ast.exprs[head]
                            && let Var(var) = self.ast.exprs[expr]
                        {
                            self.memory.bind_var(var, value);
                            args = tail;
                        } else {
                            panic!("This should be prevented by type checking.")
                        };
                    }
                    self.call(expr);
                    Some(())
                } else {
                    self.call(args_ptr);
                    Some(())
                }
            }
            ExprNode::Let { var, t, expr } => {
                if self.previous_pc == expr {
                    let expr_value = self.values_stack.pop().unwrap();
                    let ExprNode::Var(var) = self.ast.exprs[var] else {
                        panic!("This should be prevented by type checking.")
                    };
                    self.memory.bind_var(var, expr_value);
                    self.values_stack.push(Value::Constant(Constant::Unit));
                    self.ret()
                } else {
                    self.call(expr);
                    Some(())
                }
            }
            ExprNode::Assign { index, expr } => {
                if self.previous_pc == index {
                    self.call(expr);
                    Some(())
                } else if self.previous_pc == expr {
                    let expr_value = self.values_stack.pop().unwrap();
                    let index_values = self.values_stack.pop().unwrap();
                    let Value::Array(index_values) = index_values else {
                        panic!("This should be prevented by type checking.")
                    };
                    let var = self.get_var_from_index(index);
                    self.memory.replace_method(var, index_values, expr_value);
                    self.ret()
                } else {
                    self.call(index);
                    Some(())
                }
            }
            ExprNode::While { cond, body } => {
                if self.previous_pc == cond {
                    let cond_value = self.values_stack.pop().unwrap();
                    if let Value::Constant(Constant::Bool(true)) = cond_value {
                        self.call(body);
                        Some(())
                    } else if let Value::Constant(Constant::Bool(false)) = cond_value {
                        self.values_stack.push(Value::Constant(Constant::Unit));
                        self.ret()
                    } else {
                        panic!("Expecting a boolean but got '{}'.", cond_value)
                    }
                } else if self.previous_pc == body {
                    let _ = self.values_stack.pop().unwrap();
                    self.call(cond);
                    Some(())
                } else {
                    self.call(cond);
                    Some(())
                }
            }
            ExprNode::If { cond, yes, no } => {
                if self.previous_pc == cond {
                    let cond_value = self.values_stack.pop().unwrap();
                    if let Value::Constant(Constant::Bool(true)) = cond_value {
                        self.call(yes);
                        Some(())
                    } else if let Value::Constant(Constant::Bool(false)) = cond_value {
                        self.call(no);
                        Some(())
                    } else {
                        panic!("Expecting a boolean but got '{}'.", cond_value)
                    }
                } else if self.previous_pc == yes || self.previous_pc == no {
                    self.ret()
                } else {
                    self.call(cond);
                    Some(())
                }
            }
            ExprNode::Const(constant) => {
                self.values_stack.push(Value::Constant(constant));
                self.ret()
            }
            ExprNode::BinaryOp { lhs, op, rhs } => {
                if self.previous_pc == lhs {
                    self.call(rhs);
                    Some(())
                } else if self.previous_pc == rhs {
                    let rhs_value = self.values_stack.pop().unwrap();
                    let lhs_value = self.values_stack.pop().unwrap();
                    self.values_stack
                        .push(lhs_value.compute_binary(op, rhs_value));
                    self.ret()
                } else {
                    self.call(lhs);
                    Some(())
                }
            }
            ExprNode::UnaryOp { lhs, op } => {
                if self.previous_pc == lhs {
                    let lhs_value = self.values_stack.pop().unwrap();
                    self.values_stack.push(lhs_value.compute_unary(op));
                    self.ret()
                } else {
                    self.call(lhs);
                    Some(())
                }
            }
            ExprNode::TypeInfo { expr, .. } => {
                if self.previous_pc == expr {
                    self.ret()
                } else {
                    self.call(expr);
                    Some(())
                }
            }
            ExprNode::EmptyTuple => {
                self.values_stack.push(Value::Array(VecDeque::new()));
                self.ret()
            }
            ExprNode::EmptyVec => {
                self.values_stack.push(Value::Array(VecDeque::new()));
                self.ret()
            }
            ExprNode::Unknown => {
                panic!("This should be prevented by type checking.")
            }
        }
    }
}
